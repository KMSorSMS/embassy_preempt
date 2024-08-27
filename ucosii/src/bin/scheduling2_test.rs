#![no_main]
#![no_std]
#![feature(impl_trait_in_assoc_type)]

use core::ffi::c_void;

#[cfg(feature = "defmt")]
use defmt::info;
// <- derive attribute
use ucosii::os_core::{OSInit, OSStart};
use ucosii::os_task::{AsyncOSTaskCreate, SyncOSTaskCreate};
use ucosii::os_time::timer::Timer;
use ucosii::os_time::OSTimeDly;
// use ucosii::{self as _};

// 1s
const LONG_TIME: u64 = 100000;
// 1 ms
const MID_TIME: u64 = 10000;
// 1 us
const SHORT_TIME: u64 = 1000;

#[cortex_m_rt::entry]
fn main_test() -> ! {
    loop {
        test_basic_schedule();
    }
}
fn test_basic_schedule() {
    // os初始化
    OSInit();
    // 创建4个任务
    AsyncOSTaskCreate(task1, 0 as *mut c_void, 0 as *mut usize, 10);
    SyncOSTaskCreate(task2, 0 as *mut c_void, 0 as *mut usize, 11);
    AsyncOSTaskCreate(task3, 0 as *mut c_void, 0 as *mut usize, 12);
    SyncOSTaskCreate(task4, 0 as *mut c_void, 0 as *mut usize, 13);
    // 启动os
    OSStart();
}

async fn task1(_args: *mut c_void) {
    loop{
        // 任务1
        #[cfg(feature = "defmt")]
        info!("---task1 begin---");
        Timer::after_ticks(SHORT_TIME).await;
        #[cfg(feature = "defmt")]
        info!("---task1 mid---");
        Timer::after_ticks(MID_TIME).await;
        #[cfg(feature = "defmt")]
        info!("---task1 end---");
        Timer::after_ticks(LONG_TIME).await;
    }
}
fn task2(_args: *mut c_void) {
    loop{
        // 任务2
        #[cfg(feature = "defmt")]
        info!("---task2 begin---");
        OSTimeDly(SHORT_TIME);
        #[cfg(feature = "defmt")]
        info!("---task2 mid---");
        OSTimeDly(MID_TIME);
        #[cfg(feature = "defmt")]
        info!("---task2 end---");
        OSTimeDly(LONG_TIME);
    }
}
async fn task3(_args: *mut c_void) {
    // 任务3
    loop {
        //
        #[cfg(feature = "defmt")]
        info!("---task3 begin---");
        Timer::after_ticks(LONG_TIME).await;
        #[cfg(feature = "defmt")]
        info!("---task3 mid---");
        Timer::after_ticks(MID_TIME).await;
        #[cfg(feature = "defmt")]
        info!("---task3 end---");
        Timer::after_ticks(SHORT_TIME).await;
    }
}
fn task4(_args: *mut c_void) {
    loop{
        // 任务4
        #[cfg(feature = "defmt")]
        info!("---task4 begin---");
        OSTimeDly(LONG_TIME);
        #[cfg(feature = "defmt")]
        info!("---task4 mid---");
        OSTimeDly(MID_TIME);
        #[cfg(feature = "defmt")]
        info!("---task4 end---");
        OSTimeDly(SHORT_TIME);
    }
}
