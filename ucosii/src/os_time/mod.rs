use crate::executor::GlobalSyncExecutor;
use crate::port::time_driver::{Driver, RTC_DRIVER};
use crate::port::INT32U;
/// the mod of duration of uC/OS-II kernel
pub mod duration;
/// the mod of instant of uC/OS-II kernel
pub mod instant;
/// the mod of timer of uC/OS-II kernel
pub mod timer;

/// init the Timer as the Systick
pub fn OSTimerInit() {
    RTC_DRIVER.init();
}

/// we have to make this delay acting like preemptive delay
pub fn OSTimeDly(_ticks: INT32U) {
    critical_section::with(|_| {
        let cur_task = GlobalSyncExecutor.as_ref().unwrap().OSTCBCur.get_mut();
        unsafe {
            // first we set the task's expire time
            cur_task.expires_at.set(RTC_DRIVER.now() + _ticks as u64);
            // add the task to the timer queue
            let next_expire = GlobalSyncExecutor.as_ref().unwrap().timer_queue.update(*cur_task);
            if next_expire < *GlobalSyncExecutor.as_ref().unwrap().timer_queue.set_time.get_unmut() {
                GlobalSyncExecutor.as_ref().unwrap().timer_queue.set_time.set(next_expire);
            }
            // set the task unready
            GlobalSyncExecutor.as_ref().unwrap().set_task_unready(*cur_task);
        }
    });
    // call the interrupt poll
    critical_section::with(|_| unsafe { GlobalSyncExecutor.as_ref().unwrap().set_highrdy() });
    unsafe {
        GlobalSyncExecutor.as_ref().unwrap().interrupt_poll();
    }
}
