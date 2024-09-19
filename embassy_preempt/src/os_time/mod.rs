#[cfg(feature = "defmt")]
#[allow(unused_imports)]
use defmt::{info, trace};

use crate::executor::{wake_task_no_pend, GlobalSyncExecutor};
use crate::port::time_driver::{Driver, RTC_DRIVER};
use crate::port::INT64U;
/// the mod of blockdelay of uC/OS-II kernel
pub mod blockdelay;
/// the mod of duration of uC/OS-II kernel
pub mod duration;
/// the mod of instant of uC/OS-II kernel
pub mod instant;
/// the mod of timer of uC/OS-II kernel
pub mod timer;

/// init the Timer as the Systick
pub fn OSTimerInit() {
    #[cfg(feature = "defmt")]
    trace!("OSTimerInit");
    RTC_DRIVER.init();
}
/// we have to make this delay acting like preemptive delay
pub fn OSTimeDly(_ticks: INT64U) {
    #[cfg(feature = "defmt")]
    trace!("OSTimeDly");
    unsafe {
        delay_tick(_ticks);
    }
}

pub(crate) unsafe fn delay_tick(_ticks: INT64U) {
    // by noah：Remove tasks from the ready queue in advance to facilitate subsequent unified operations
    let executor = GlobalSyncExecutor.as_ref().unwrap();
    let task = executor.OSTCBCur.get_mut();
    task.expires_at.set(RTC_DRIVER.now() + _ticks);
    // update timer
    let mut next_expire = critical_section::with(|_| {
        executor.set_task_unready(*task);
        critical_section::with(|_| executor.timer_queue.update(*task))
    });
    #[cfg(feature = "defmt")]
    trace!("in delay_tick the next expire is {:?}", next_expire);
    if critical_section::with(|_| {
        if next_expire < *executor.timer_queue.set_time.get_unmut() {
            executor.timer_queue.set_time.set(next_expire);
            true
        } else {
            // if the next_expire is not less than the set_time, it means the expire dose not arrive, or the task
            // dose not expire a timestamp so we should set the task unready
            false
        }
    }) {
        // by noah：if the set alarm return false, it means the expire arrived.
        // So we can not set the **task which is waiting for the next_expire** as unready
        // The **task which is waiting for the next_expire** must be current task
        // we must do this until we set the alarm successfully or there is no alarm required
        while !RTC_DRIVER.set_alarm(executor.alarm, next_expire) {
            // by noah: if set alarm failed, it means the expire arrived, so we should not set the task unready
            // we should **dequeue the task** from time_queue, **clear the set_time of the time_queue** and continue the loop
            // (just like the operation in alarm_callback)
            executor
                .timer_queue
                .dequeue_expired(RTC_DRIVER.now(), wake_task_no_pend);
            // then we need to set a new alarm according to the next expiration time
            next_expire = unsafe { executor.timer_queue.next_expiration() };
            #[cfg(feature = "defmt")]
            trace!("in delay_tick the next expire is {:?}", next_expire);
            // by noah：we also need to updater the set_time of the timer_queue
            executor.timer_queue.set_time.set(next_expire);
        }
    }
    // find the highrdy
    if critical_section::with(|_| {
        executor.set_highrdy();
        executor.OSPrioHighRdy != executor.OSPrioCur
    }) {
        // call the interrupt poll
        GlobalSyncExecutor.as_ref().unwrap().interrupt_poll();
        #[cfg(feature = "defmt")]
        trace!("end the delay");
    }
}