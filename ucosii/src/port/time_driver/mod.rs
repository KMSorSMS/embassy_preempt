/*
*********************************************************************************************************
*                                 The mods that os_core.rs depends on
*********************************************************************************************************
*/
use core::cell::Cell;
use core::sync::atomic::{compiler_fence, AtomicU32, AtomicU8, Ordering};
use core::{mem, ptr};

use cortex_m::peripheral::NVIC;
use critical_section::{CriticalSection, Mutex};
#[cfg(feature = "defmt")]
use defmt::info;
use stm32_metapac::flash::vals::Latency;
use stm32_metapac::rcc::vals::*;
use stm32_metapac::timer::{regs, vals};
use stm32_metapac::{Interrupt, FLASH, RCC};

use crate::cfg::{APB_HZ, TICK_HZ};
use crate::executor::waker;
use crate::port::{BOOLEAN, INT16U, INT32U, INT64U, INT8U, TIMER, USIZE};

#[cfg(any(
    feature = "time_driver_tim9",
    feature = "time_driver_tim12",
    feature = "time_driver_tim15",
    feature = "time_driver_tim21",
    feature = "time_driver_tim22"
))]
const ALARM_COUNT: USIZE = 1;
#[cfg(not(any(
    feature = "time_driver_tim9",
    feature = "time_driver_tim12",
    feature = "time_driver_tim15",
    feature = "time_driver_tim21",
    feature = "time_driver_tim22"
)))]
const ALARM_COUNT: USIZE = 3;
// define the Alarm Interrupt
#[cfg(feature = "time_driver_tim3")]
#[no_mangle]
/// TIM3 interrupt handler
pub extern "C" fn TIM3() {
    RTC_DRIVER.on_interrupt();
}
/*
*********************************************************************************************************
*                                           type definitions
*********************************************************************************************************
*/
const DISABLE: bool = false;
const ENABLE: bool = true;

struct AlarmState {
    timestamp: Cell<INT64U>,
    // This is really a Option<(fn(*mut ()), *mut ())>
    // but fn pointers aren't allowed in const yet
    callback: Cell<*const ()>,
    ctx: Cell<*mut ()>,
}
#[derive(Clone, Copy)]
/// Handle to an alarm.
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
    fn set_alarm_callback(&self, alarm: AlarmHandle, callback: fn(*mut ()), ctx: *mut ());

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
    pub(crate) fn init(&'static self) {
        // rcc config
        rcc_init();
        // enable the Timer Driver
        enable_Timer();

        // disable the Timer
        TIMER.cr1().modify(|w| w.set_cen(false));
        // clear the cnt num
        TIMER.cnt().write(|w| w.set_cnt(0));

        // calculate the psc
        let psc = (APB_HZ / TICK_HZ) as INT32U - 1;
        let psc: INT16U = match psc.try_into() {
            Err(_) => panic!("psc division overflow: {}", psc),
            Ok(n) => n,
        };
        // set the psc
        TIMER.psc().write_value(psc);
        // set the auto reload value(Timer 3 is INT16U)
        TIMER.arr().write(|w| w.set_arr(INT16U::MAX));

        // Set URS, generate update and clear URS
        // by noah： when set ug bit, a update event will be generated immediately.
        // In the update event, the counter(including prescaler counter) will be clear.
        // So in this way, we can init the counter by this way
        TIMER.cr1().modify(|w| w.set_urs(vals::Urs::COUNTERONLY));
        TIMER.egr().write(|w| w.set_ug(true));
        TIMER.cr1().modify(|w| w.set_urs(vals::Urs::ANYEVENT));

        // Mid-way point, there will be a cc(capture/compare) interrupt at 0x8000
        TIMER.ccr(0).write(|w| w.set_ccr(0x8000));

        TIMER.dier().write(|w| {
            // Enable overflow
            w.set_uie(true);
            // and half-overflow interrupts
            w.set_ccie(0, true);
        });

        // by noah：the InterruptNumber trait is implemented by the stm32_metapac crate
        // so in embassy, the InterruptExt trait will be implemented for the interrupt in pac
        #[cfg(feature = "time_driver_tim3")]
        NVIC::unpend(Interrupt::TIM3);
        #[cfg(feature = "time_driver_tim3")]
        unsafe {
            compiler_fence(Ordering::SeqCst);
            NVIC::unmask(Interrupt::TIM3);
        }
        // <T as GeneralInstance1Channel>::CaptureCompareInterrupt::unpend();
        // unsafe { <T as GeneralInstance1Channel>::CaptureCompareInterrupt::enable() };

        // enable the Timer
        TIMER.cr1().modify(|w| w.set_cen(ENABLE));
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
                    #[cfg(feature = "defmt")]
                    info!("the alarm is triggered!!!");
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
        let counter = TIMER.cnt().read().cnt();
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

    fn set_alarm_callback(&self, alarm: AlarmHandle, callback: fn(*mut ()), ctx: *mut ()) {
        critical_section::with(|cs| {
            let alarm = self.get_alarm(cs, alarm);

            alarm.callback.set(callback as *const ());
            alarm.ctx.set(ctx);
        })
    }

    fn set_alarm(&self, alarm: AlarmHandle, timestamp: INT64U) -> bool {
        #[cfg(feature = "defmt")]
        info!("set the alarm");
        let n = alarm.id() as usize;
        // by noah：check the timestamp. if timestamp is INT64U::MAX, there is no need to set the alarm
        if timestamp == INT64U::MAX {
            // return true to indicate that there is no need to set the alarm, poll can execute directly
            // before return, unset the ccie bit
            critical_section::with(|cs| {
                let alarm = self.get_alarm(cs, alarm);
                alarm.timestamp.set(u64::MAX);
                TIMER.dier().modify(|w| w.set_ccie(n + 1, false));
            });
            return true;
        }
        critical_section::with(|cs| {
            let alarm = self.get_alarm(cs, alarm);
            // by noah：for timestamp is INT64U, so I just copy it here
            // when the timestamp is less than the alarm's timestamp, the alarm will be set.
            // if the timestamp is large than the allarm's timestamp, the alarm will not be set.
            // because the allarm's timestamp has been set in the last call of set_alarm
            // by noah：we will check the timestamp outside this func
            // if timestamp<alarm.timestamp.get(){
            alarm.timestamp.set(timestamp);
            let t = self.now();
            if timestamp <= t {
                // If alarm timestamp has passed the alarm will not fire.
                // Disarm the alarm and return `false` to indicate that.
                TIMER.dier().modify(|w| w.set_ccie(n + 1, false));

                alarm.timestamp.set(u64::MAX);

                return false;
            }

            // Write the CCR value regardless of whether we're going to enable it now or not.
            // This way, when we enable it later, the right value is already set.
            TIMER.ccr(n + 1).write(|w| w.set_ccr(timestamp as u16));

            // Enable it if it'll happen soon. Otherwise, `next_period` will enable it.
            let diff = timestamp - t;
            TIMER.dier().modify(|w| w.set_ccie(n + 1, diff < 0xc000));

            // Reevaluate if the alarm timestamp is still in the future
            let t = self.now();
            if timestamp <= t {
                // If alarm timestamp has passed since we set it, we have a race condition and
                // the alarm may or may not have fired.
                // Disarm the alarm and return `false` to indicate that.
                // It is the caller's responsibility to handle this ambiguity.
                TIMER.dier().modify(|w| w.set_ccie(n + 1, false));

                alarm.timestamp.set(u64::MAX);

                return false;
            }
            // }
            // We're confident the alarm will ring in the future.
            // if timestamp > alarm.timestamp, this func will not set the alarm and will return directly
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
pub(crate) static RTC_DRIVER: RtcDriver = RtcDriver {
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

/// set the rcc of the Timer
pub fn rcc_init() {
    RCC.cr().modify(|v| {
        // disable PLL
        v.set_pllon(DISABLE);
        // disable PLL2S
        v.set_plli2son(DISABLE);
    });
    RCC.pllcfgr().modify(|v| {
        // set PLLM=4
        v.set_pllm(Pllm::DIV4);
        // set PLLN=84
        v.set_plln(Plln::MUL84);
        // set PLLP=2
        v.set_pllp(Pllp::DIV2);
        // set PLLQ=4
        v.set_pllq(Pllq::DIV4);
        // set the HSE as the PLL source
        v.set_pllsrc(Pllsrc::HSE);
    });
    RCC.cfgr().modify(|v| {
        // set the frequency division coefficient of AHB as 1
        v.set_hpre(Hpre::DIV1);
        // set the frequency division coefficient of APB1 as 2
        v.set_ppre1(Ppre::DIV2);
        // set the frequency division coefficient of APB2 as 1
        v.set_ppre2(Ppre::DIV1);
    });
    RCC.cr().modify(|v| {
        // enable the HSE
        v.set_hseon(ENABLE);
        // enable the PLL
        v.set_pllon(ENABLE);
        // enable the PLL2S
        v.set_plli2son(ENABLE);
    });
    // check the state of HSE, PLL, PLL2S
    while !RCC.cr().read().hserdy() || !RCC.cr().read().pllrdy() || !RCC.cr().read().plli2srdy() {}
    // enable FLASH prefetch buffer
    FLASH.acr().modify(|v| v.set_prften(ENABLE));
    // set the wait state of FLASH as 2
    FLASH.acr().modify(|v| v.set_latency(Latency::WS2));
    // set the system clock as PLL
    RCC.cfgr().modify(|v| v.set_sw(Sw::PLL1_P));
    // close the HSI
    RCC.cr().modify(|v| v.set_hsion(DISABLE));
}

fn enable_Timer() {
    #[cfg(feature = "time_driver_tim3")]
    // by noah: in current project, we use Timer 3 as the time driver
    RCC.apb1enr().modify(|v| v.set_tim3en(ENABLE));
    #[cfg(not(feature = "time_driver_tim3"))]
    panic!("the Timer is not surpport. You can add it in Func enable_Timer()");
}

// by noah: here the period is shifted 15 bit because period will be increased when the counter is overflowed or half-overflowed
fn calc_now(period: INT32U, counter: INT16U) -> INT64U {
    ((period as INT64U) << 15) + ((counter as u32 ^ ((period & 1) << 15)) as u64)
}

#[no_mangle]
/// Schedule the given waker to be woken at `at`.
pub fn _embassy_time_schedule_wake(at: u64, waker: &core::task::Waker) {
    let task = waker::task_from_waker(waker);
    let task = task.header();
    unsafe {
        let expires_at = task.expires_at.get();
        task.expires_at.set(expires_at.min(at));
    }
}
