/*
*********************************************************************************************************
*                                 The mods that os_core.rs depends on
*********************************************************************************************************
*/

use core::{cell::Cell, ptr, sync::atomic::{AtomicU32, AtomicU8}};

use critical_section::Mutex;
use stm32_metapac::{rcc::vals::{Pllm, Plln}, timer::TimGp16, RCC};

use crate::{cfg::TICK_HZ, port::{BOOLEAN, INT64U, INT8U, TIMER, USIZE}};

#[cfg(any(feature="time_driver_tim9", feature="time_driver_tim12", feature="time_driver_tim15", feature="time_driver_tim21", feature="time_driver_tim22"))]
const ALARM_COUNT: USIZE = 1;
#[cfg(not(any(feature="time_driver_tim9", feature="time_driver_tim12", feature="time_driver_tim15", feature="time_driver_tim21", feature="time_driver_tim22")))]
const ALARM_COUNT: USIZE = 3;

/*
*********************************************************************************************************
*                                           type definitions
*********************************************************************************************************
*/
const DISABLE:bool=false;
const ENABLE:bool=true;


struct AlarmState {
    timestamp: Cell<INT64U>,
    // This is really a Option<(fn(*mut ()), *mut ())>
    // but fn pointers aren't allowed in const yet
    callback: Cell<*const ()>,
    ctx: Cell<*mut ()>,
}

pub struct AlarmHandle {
    id: INT8U,
}

pub(crate) struct RtcDriver {
    /// Number of 2^15 periods elapsed since boot.
    period: AtomicU32,
    alarm_count: AtomicU8,
    /// Timestamp at which to fire alarm. u64::MAX if no alarm is scheduled.
    alarms: Mutex<[AlarmState; ALARM_COUNT]>,
    #[cfg(feature = "low-power")]
    rtc: Mutex<CriticalSectionRawMutex, Cell<Option<&'static Rtc>>>,
}

/// Time driver
pub trait Driver: Send + Sync + 'static {
    /// Return the current timestamp in ticks.
    ///
    /// Implementations MUST ensure that:
    /// - This is guaranteed to be monotonic, i.e. a call to now() will always return
    ///   a greater or equal value than earler calls. Time can't "roll backwards".
    /// - It "never" overflows. It must not overflow in a sufficiently long time frame, say
    ///   in 10_000 years (Human civilization is likely to already have self-destructed
    ///   10_000 years from now.). This means if your hardware only has 16bit/32bit timers
    ///   you MUST extend them to 64-bit, for example by counting overflows in software,
    ///   or chaining multiple timers together.
    fn now(&self) -> INT64U;

    /// Try allocating an alarm handle. Returns None if no alarms left.
    /// Initially the alarm has no callback set, and a null `ctx` pointer.
    ///
    /// # Safety
    /// It is UB to make the alarm fire before setting a callback.
    unsafe fn allocate_alarm(&self) -> Option<AlarmHandle>;

    /// Sets the callback function to be called when the alarm triggers.
    /// The callback may be called from any context (interrupt or thread mode).
    /// by noah：this func will not be used in the current project
    // fn set_alarm_callback(&self, alarm: AlarmHandle, callback: fn(*mut ()), ctx: *mut ());

    /// Sets an alarm at the given timestamp. When the current timestamp reaches the alarm
    /// timestamp, the provided callback function will be called.
    ///
    /// The `Driver` implementation should guarantee that the alarm callback is never called synchronously from `set_alarm`.
    /// Rather - if `timestamp` is already in the past - `false` should be returned and alarm should not be set,
    /// or alternatively, the driver should return `true` and arrange to call the alarm callback as soon as possible, but not synchronously.
    /// There is a rare third possibility that the alarm was barely in the future, and by the time it was enabled, it had slipped into the
    /// past.  This is can be detected by double-checking that the alarm is still in the future after enabling it; if it isn't, `false`
    /// should also be returned to indicate that the callback may have been called already by the alarm, but it is not guaranteed, so the
    /// caller should also call the callback, just like in the more common `false` case. (Note: This requires idempotency of the callback.)
    ///
    /// When callback is called, it is guaranteed that now() will return a value greater or equal than timestamp.
    ///
    /// Only one alarm can be active at a time for each AlarmHandle. This overwrites any previously-set alarm if any.
    fn set_alarm(&self, alarm: AlarmHandle, timestamp: INT64U) -> BOOLEAN;
}

/*
*********************************************************************************************************
*                                              implentations
*********************************************************************************************************
*/

unsafe impl Send for AlarmState {}

impl AlarmState {
    const fn new() -> Self {
        Self {
            timestamp: Cell::new(INT64U::MAX),
            callback: Cell::new(ptr::null()),
            ctx: Cell::new(ptr::null_mut()),
        }
    }
}

impl AlarmHandle {
    /// Create an AlarmHandle
    ///
    /// Safety: May only be called by the current global Driver impl.
    /// The impl is allowed to rely on the fact that all `AlarmHandle` instances
    /// are created by itself in unsafe code (e.g. indexing operations)
    pub unsafe fn new(id: u8) -> Self {
        Self { id }
    }

    /// Get the ID of the AlarmHandle.
    pub fn id(&self) -> u8 {
        self.id
    }
}

impl RtcDriver {
    fn init(&'static self) {
        // let r = regs_gp16();
        rcc_clock_init();
        // rcc::enable_and_reset_with_cs::<TIMER>();

        let timer_freq = T::frequency();

        TIMER.cr1().modify(|w| w.set_cen(false));
        TIMER.cnt().write(|w| w.set_cnt(0));

        let psc = timer_freq.0 / TICK_HZ as u32 - 1;
        let psc: u16 = match psc.try_into() {
            Err(_) => panic!("psc division overflow: {}", psc),
            Ok(n) => n,
        };

        TIMER.psc().write_value(psc);
        TIMER.arr().write(|w| w.set_arr(u16::MAX));

        // Set URS, generate update and clear URS
        TIMER.cr1().modify(|w| w.set_urs(vals::Urs::COUNTERONLY));
        TIMER.egr().write(|w| w.set_ug(true));
        TIMER.cr1().modify(|w| w.set_urs(vals::Urs::ANYEVENT));

        // Mid-way point
        TIMER.ccr(0).write(|w| w.set_ccr(0x8000));

        // Enable overflow and half-overflow interrupts
        TIMER.dier().write(|w| {
            w.set_uie(true);
            w.set_ccie(0, true);
        });

        <T as GeneralInstance1Channel>::CaptureCompareInterrupt::unpend();
        unsafe { <T as GeneralInstance1Channel>::CaptureCompareInterrupt::enable() };

        TIMER.cr1().modify(|w| w.set_cen(true));
    }

    fn on_interrupt(&self) {
        // let r = regs_gp16();

        // XXX: reduce the size of this critical section ?
        critical_section::with(|cs| {
            let sr = TIMER.sr().read();
            let dier = TIMER.dier().read();

            // Clear all interrupt flags. Bits in SR are "write 0 to clear", so write the bitwise NOT.
            // Other approaches such as writing all zeros, or RMWing won't work, they can
            // miss interrupts.
            TIMER.sr().write_value(regs::SrGp16(!sr.0));

            // Overflow
            if sr.uif() {
                self.next_period();
            }

            // Half overflow
            if sr.ccif(0) {
                self.next_period();
            }

            for n in 0..ALARM_COUNT {
                if sr.ccif(n + 1) && dier.ccie(n + 1) {
                    self.trigger_alarm(n, cs);
                }
            }
        })
    }

    fn next_period(&self) {
        // let r = regs_gp16();

        // We only modify the period from the timer interrupt, so we know this can't race.
        let period = self.period.load(Ordering::Relaxed) + 1;
        self.period.store(period, Ordering::Relaxed);
        let t = (period as u64) << 15;

        critical_section::with(move |cs| {
            TIMER.dier().modify(move |w| {
                for n in 0..ALARM_COUNT {
                    let alarm = &self.alarms.borrow(cs)[n];
                    let at = alarm.timestamp.get();

                    if at < t + 0xc000 {
                        // just enable it. `set_alarm` has already set the correct CCR val.
                        w.set_ccie(n + 1, true);
                    }
                }
            })
        })
    }

    fn get_alarm<'a>(&'a self, cs: CriticalSection<'a>, alarm: AlarmHandle) -> &'a AlarmState {
        // safety: we're allowed to assume the AlarmState is created by us, and
        // we never create one that's out of bounds.
        unsafe { self.alarms.borrow(cs).get_unchecked(alarm.id() as usize) }
    }

    fn trigger_alarm(&self, n: usize, cs: CriticalSection) {
        let alarm = &self.alarms.borrow(cs)[n];
        alarm.timestamp.set(u64::MAX);

        // Call after clearing alarm, so the callback can set another alarm.

        // safety:
        // - we can ignore the possibility of `f` being unset (null) because of the safety contract of `allocate_alarm`.
        // - other than that we only store valid function pointers into alarm.callback
        let f: fn(*mut ()) = unsafe { mem::transmute(alarm.callback.get()) };
        f(alarm.ctx.get());
    }
}



impl Driver for RtcDriver {
    fn now(&self) -> u64 {
        // let r = regs_gp16();

        let period = self.period.load(Ordering::Relaxed);
        compiler_fence(Ordering::Acquire);
        let counter = r.cnt().read().cnt();
        calc_now(period, counter)
    }

    unsafe fn allocate_alarm(&self) -> Option<AlarmHandle> {
        critical_section::with(|_| {
            let id = self.alarm_count.load(Ordering::Relaxed);
            if id < ALARM_COUNT as u8 {
                self.alarm_count.store(id + 1, Ordering::Relaxed);
                Some(AlarmHandle::new(id))
            } else {
                None
            }
        })
    }

    // fn set_alarm_callback(&self, alarm: AlarmHandle, callback: fn(*mut ()), ctx: *mut ()) {
    //     critical_section::with(|cs| {
    //         let alarm = self.get_alarm(cs, alarm);

    //         alarm.callback.set(callback as *const ());
    //         alarm.ctx.set(ctx);
    //     })
    // }

    fn set_alarm(&self, alarm: AlarmHandle, timestamp: u64) -> bool {
        critical_section::with(|cs| {
            // let r = regs_gp16();

            let n = alarm.id() as usize;
            let alarm = self.get_alarm(cs, alarm);
            alarm.timestamp.set(timestamp);

            let t = self.now();
            if timestamp <= t {
                // If alarm timestamp has passed the alarm will not fire.
                // Disarm the alarm and return `false` to indicate that.
                r.dier().modify(|w| w.set_ccie(n + 1, false));

                alarm.timestamp.set(u64::MAX);

                return false;
            }

            // Write the CCR value regardless of whether we're going to enable it now or not.
            // This way, when we enable it later, the right value is already set.
            r.ccr(n + 1).write(|w| w.set_ccr(timestamp as u16));

            // Enable it if it'll happen soon. Otherwise, `next_period` will enable it.
            let diff = timestamp - t;
            r.dier().modify(|w| w.set_ccie(n + 1, diff < 0xc000));

            // Reevaluate if the alarm timestamp is still in the future
            let t = self.now();
            if timestamp <= t {
                // If alarm timestamp has passed since we set it, we have a race condition and
                // the alarm may or may not have fired.
                // Disarm the alarm and return `false` to indicate that.
                // It is the caller's responsibility to handle this ambiguity.
                r.dier().modify(|w| w.set_ccie(n + 1, false));

                alarm.timestamp.set(u64::MAX);

                return false;
            }

            // We're confident the alarm will ring in the future.
            true
        })
    }
}

/*
*********************************************************************************************************
*                                           var declaration
*********************************************************************************************************
*/

#[allow(clippy::declare_interior_mutable_const)]
const ALARM_STATE_NEW: AlarmState = AlarmState::new();

/// the global RTC driver
static DRIVER: RtcDriver = RtcDriver {
    period: AtomicU32::new(0),
    alarm_count: AtomicU8::new(0),
    alarms: Mutex::new([ALARM_STATE_NEW; ALARM_COUNT]),
    #[cfg(feature = "low-power")]
    rtc: Mutex::const_new(CriticalSectionRawMutex::new(), Cell::new(None)),
};

/*
*********************************************************************************************************
*                                           auxiliary function
*********************************************************************************************************
*/

// get the Timer instance
// fn regs_gp16() -> TimGp16 {
//     unsafe { TimGp16::from_ptr(TIMER::regs()) }
// }

// set the rcc of the Timer
fn rcc_clock_init(){
    RCC.cr().modify(|v| {
        // disable PLL
        v.set_pllon(DISABLE);
        // disable PLL2S
        v.set_plli2son(DISABLE);
    });
    RCC.pllcfgr().modify(|v|{
        // PLLP=2, PLLQ=4，并且设置HSE为PLL的输入源
        // set PLLM=4
        v.set_pllm(Pllm::DIV4);
        // set PLLN=84
        v.set_plln(Plln::MUL84);

    })
}