// this part can be rewrite as the Semaphore part

use core::cell::Cell;
use core::sync::atomic::{compiler_fence, Ordering};

use cortex_m::peripheral::NVIC;
use critical_section::Mutex;
#[cfg(feature = "defmt")]
use defmt::{info, trace};
// use critical_section::{CriticalSection, Mutex};
#[allow(unused_imports)]
use stm32_metapac::{gpio::vals, EXTI, GPIOA, GPIOC, RCC, SYSCFG};

use super::{DISENABLE, ENABLE};
use crate::executor::{wake_task_no_pend, GlobalSyncExecutor, OS_TCB_REF};
use crate::pac::Interrupt;
use crate::util::SyncUnsafeCell;

/// async bottom
pub mod Bottom;

struct BottomState {
    // the task await on the bottom(for now, only one task)
    task: SyncUnsafeCell<Option<OS_TCB_REF>>,
    // This is really a Option<(fn(*mut ()), *mut ())>
    // but fn pointers aren't allowed in const yet
    /// the callback func of the bottom
    #[allow(unused)]
    callback: Cell<*const ()>,
    /// the argument to the callback
    #[allow(unused)]
    ctx: Cell<*mut ()>,
}

/// this trait can be change into a sem driver in the future
// pub trait EventDriver {
//     /// new a bottom(a sem in the future)
//     fn allocate_sem()->Option<SemHandle>;
//     /// set the callback of the bottom(will be called in the interrupt)
//     fn set_callback(&self, sem:SemHandle, callback: fn(*mut ()), ctx: *mut ());
// }

///  the bottom driver
pub struct BotDriver {
    // by noah: we can set the bottom to an array (indicate a pin) in the future
    bottoms: Mutex<BottomState>,
}

pub(crate) static BOT_DRIVER: BotDriver = BotDriver {
    bottoms: Mutex::new(BottomState {
        task: SyncUnsafeCell::new(None),
        callback: Cell::new(core::ptr::null()),
        ctx: Cell::new(core::ptr::null_mut()),
    }),
};

#[no_mangle]
#[cfg(feature = "GPIOC")]
/// EXTI15_10 interrupt handler
pub extern "C" fn EXTI15_10() {
    #[cfg(feature = "defmt")]
    info!("EXTI15_10");
    BOT_DRIVER.on_interrupt();
    #[cfg(feature = "defmt")]
    info!("exit_EXTI15_10");
}

/// EXTI15_10 interrupt handler
#[cfg(feature = "GPIOA")]
#[no_mangle]
pub extern "C" fn EXTI0() {
    #[cfg(feature = "defmt")]
    info!("EXTI0");
    BOT_DRIVER.on_interrupt();
}

unsafe impl Send for BotDriver {}
unsafe impl Sync for BotDriver {}

// use PC13 as the source of EXIT13
impl BotDriver {
    pub(crate) fn init(&'static self) {
        #[cfg(feature = "defmt")]
        trace!("init of BotDriver");
        // gpio config
        bottom_init();

        // set the interrupt
        set_Interrupt();
    }

    fn on_interrupt(&self) {
        // disable the interrupt
        #[cfg(feature = "GPIOC")]
        EXTI.imr(0).modify(|w| {
            // mask the EXTI13
            w.set_line(13, DISENABLE)
        });

        #[cfg(feature = "GPIOC")]
        // clear the pending bit in EXTI
        EXTI.pr(0).modify(|w| {
            // This bit is cleared by programming it to ‘1’.
            w.set_line(13, ENABLE)
        });
        
        #[cfg(feature = "GPIOA")]
        // clear the pending bit in EXTI
        EXTI.pr(0).modify(|w| {
            // This bit is cleared by programming it to ‘1’.
            w.set_line(0, ENABLE)
        });

        // clear the pedning bit in NVIC
        #[cfg(feature = "GPIOC")]
        NVIC::unpend(Interrupt::EXTI15_10);

        #[cfg(feature = "GPIOA")]
        NVIC::unpend(Interrupt::EXTI0);

        // XXX: reduce the size of this critical section ?
        critical_section::with(|cs| {
            // clear the pedning bit in NVIC
            // #[cfg(feature="GPIOC")]
            // NVIC::unpend(Interrupt::EXTI15_10);

            // #[cfg(feature = "GPIOA")]
            // NVIC::unpend(Interrupt::EXTI0);

            // self.trigger_alarm(cs);
            // inline the trigger_alarm
            let bottom = self.bottoms.borrow(cs);
            // when the bottom is pressed, the task must be waked up
            let task: Option<OS_TCB_REF>;
            unsafe {
                task = bottom.task.get();
            }
            // wake up the task (set the task ready)
            wake_task_no_pend(task.unwrap());
            // clear the task
            unsafe {
                bottom.task.set(None);
            }
            // // disable the interrupt
            // #[cfg(feature="GPIOC")]
            // EXTI.imr(0).modify(|w|{
            //     // mask the EXTI13
            //     w.set_line(13, DISENABLE)
            // });

            // rescheduling
            unsafe { GlobalSyncExecutor.as_ref().unwrap().IntCtxSW() };
        })
    }

    /// set the task of the bottom driver
    // only after set task, the interrupt can be enable
    pub fn set_task(&self, task: OS_TCB_REF) {
        critical_section::with(|cs| {
            let bottom = self.bottoms.borrow(cs);
            unsafe {
                bottom.task.set(Some(task));
            }

            // enable the interrupt
            // clear the EXTI13 pending
            #[cfg(feature = "GPIOC")]
            EXTI.pr(0).modify(|w| {
                // This bit is cleared by programming it to ‘1’.
                w.set_line(13, ENABLE)
            });

            #[cfg(feature = "GPIOC")]
            // stm32f401 only has one EXTI, so we pass the 0 to the imr
            EXTI.imr(0).modify(|w| {
                // unmask the EXTI13
                w.set_line(13, ENABLE)
            });

            #[cfg(feature = "GPIOC")]
            EXTI.pr(0).modify(|w| {
                // This bit is cleared by programming it to ‘1’.
                w.set_line(13, ENABLE)
            });

            #[cfg(feature = "GPIOA")]
            EXTI.imr(0).modify(|w| {
                // unmask the EXTI13
                w.set_line(0, ENABLE)
            });

            #[cfg(feature = "GPIOA")]
            EXTI.pr(0).modify(|w| {
                // This bit is cleared by programming it to ‘1’.
                w.set_line(0, ENABLE)
            });
        });
    }
}

#[cfg(feature = "GPIOC")]
fn bottom_init() {
    // enable the RCC
    RCC.ahb1enr().modify(|v| {
        v.set_gpiocen(ENABLE);
    });
    // set GPIO
    GPIOC.moder().modify(|v| {
        // set mode as input
        v.set_moder(13, vals::Moder::INPUT);
    });
    // set as input, we don't need to set the otype, ospeedr, pupdr
    // GPIOC.otyper().modify(|v| {
    //     // set output type as push-pull
    //     v.set_ot(13, vals::Ot::OPENDRAIN);
    // });
    // GPIOC.ospeedr().modify(|v| {
    //     // set output speed as high
    //     v.set_ospeedr(13, vals::Ospeedr::HIGHSPEED);
    // });
    GPIOC.pupdr().modify(|v| {
        // set pull-up/pull-down as no pull-up/pull-down
        v.set_pupdr(13, vals::Pupdr::PULLDOWN);
    });
    // GPIOC.odr().modify(|v| {
    //     // set output as high
    //     v.set_odr(13, vals::Odr::HIGH);
    // });
}

#[cfg(feature = "GPIOA")]
fn bottom_init() {
    // enable the RCC
    RCC.ahb1enr().modify(|v| {
        v.set_gpioaen(true);
    });
    // set GPIO
    GPIOA.moder().modify(|v| {
        // set mode as input
        v.set_moder(0, vals::Moder::INPUT);
    });
    // set as input, we don't need to set the otype, ospeedr, pupdr
    // GPIOC.otyper().modify(|v| {
    //     // set output type as push-pull
    //     v.set_ot(13, vals::Ot::OPENDRAIN);
    // });
    // GPIOC.ospeedr().modify(|v| {
    //     // set output speed as high
    //     v.set_ospeedr(13, vals::Ospeedr::HIGHSPEED);
    // });
    GPIOA.pupdr().modify(|v| {
        // set pull-up/pull-down as no pull-up/pull-down
        v.set_pupdr(0, vals::Pupdr::PULLDOWN);
    });
    // GPIOC.odr().modify(|v| {
    //     // set output as high
    //     v.set_odr(13, vals::Odr::HIGH);
    // });
}

#[cfg(feature = "GPIOC")]
fn set_Interrupt() {
    // enable the SYSCFG
    RCC.apb2enr().modify(|v| {
        v.set_syscfgen(ENABLE);
    });

    // set pc13 as the source of EXTI13
    SYSCFG.exticr(3).modify(|w| {
        w.set_exti(1, 2);
    });

    EXTI.ftsr(0).modify(|w| {
        // set the EXTI13 as the raising edge
        w.set_line(13, ENABLE)
    });

    NVIC::unpend(Interrupt::EXTI15_10);

    // by noah：the InterruptNumber trait is implemented by the stm32_metapac crate
    // so in embassy, the InterruptExt trait will be implemented for the interrupt in pac
    unsafe {
        compiler_fence(Ordering::SeqCst);
        NVIC::unmask(Interrupt::EXTI15_10);
    }
}

#[cfg(feature = "GPIOA")]
fn set_Interrupt() {
    // enable the SYSCFG
    RCC.apb2enr().modify(|v| {
        v.set_syscfgen(ENABLE);
    });

    // set pc13 as the source of EXTI13
    SYSCFG.exticr(0).modify(|w| {
        w.set_exti(0, 0);
    });

    EXTI.rtsr(0).modify(|w| {
        // set the EXTI13 as the raising edge
        w.set_line(0, ENABLE)
    });

    NVIC::unpend(Interrupt::EXTI0);

    // by noah：the InterruptNumber trait is implemented by the stm32_metapac crate
    // so in embassy, the InterruptExt trait will be implemented for the interrupt in pac
    unsafe {
        compiler_fence(Ordering::SeqCst);
        NVIC::unmask(Interrupt::EXTI0);
    }
}
