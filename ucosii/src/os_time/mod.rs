use defmt::info;

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

/// we have to make this delay acting like preemptive delay（like soft interrupt）
pub fn OSTimeDly(_ticks: INT32U) {
    if critical_section::with(|_| {
        let cur_task = GlobalSyncExecutor.as_ref().unwrap().OSTCBCur.get_mut();
        let ticks = RTC_DRIVER.now() + _ticks as u64;
        
        if RTC_DRIVER.set_alarm(GlobalSyncExecutor.as_ref().unwrap().alarm, ticks) {
            unsafe {
                // first we set the task's expire time
                info!("set the task expire time is {}", ticks);
                cur_task.expires_at.set(ticks);
                // add the task to the timer queue
                let next_expire = GlobalSyncExecutor.as_ref().unwrap().timer_queue.update(*cur_task);
                if next_expire < *GlobalSyncExecutor.as_ref().unwrap().timer_queue.set_time.get_unmut() {
                    GlobalSyncExecutor
                        .as_ref()
                        .unwrap()
                        .timer_queue
                        .set_time
                        .set(next_expire);
                }
                // set the task unready
                GlobalSyncExecutor.as_ref().unwrap().set_task_unready(*cur_task);
            }
            true
        } else {
            false
        }
    }) {
        // call the interrupt poll
        critical_section::with(|_| unsafe { GlobalSyncExecutor.as_ref().unwrap().set_highrdy() });
        unsafe {
            GlobalSyncExecutor.as_ref().unwrap().interrupt_poll();
        }
    }
}
