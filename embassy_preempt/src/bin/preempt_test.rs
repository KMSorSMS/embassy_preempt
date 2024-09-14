#![no_main]
#![no_std]
#![feature(impl_trait_in_assoc_type)]

use core::ffi::c_void;

#[cfg(feature = "defmt")]
use defmt::info;
// <- derive attribute
use embassy_preempt::os_core::{OSInit, OSStart};
use embassy_preempt::os_task::{AsyncOSTaskCreate, SyncOSTaskCreate};
use embassy_preempt::os_time::blockdelay::delay;
use embassy_preempt::os_time::timer::Timer;
use embassy_preempt::os_time::OSTimeDly;
// use embassy_preempt::{self as _};

const LONG_TIME: usize = 10;
const MID_TIME: usize = 5;
const SHORT_TIME: usize = 3;

#[cortex_m_rt::entry]
fn test_basic_schedule() -> ! {
    // os初始化
    OSInit();
    // 创建两个任务
    SyncOSTaskCreate(task1, 0 as *mut c_void, 0 as *mut usize, 10);
    SyncOSTaskCreate(task2, 0 as *mut c_void, 0 as *mut usize, 11);
    AsyncOSTaskCreate(task3, 0 as *mut c_void, 0 as *mut usize, 12);
    SyncOSTaskCreate(task4, 0 as *mut c_void, 0 as *mut usize, 13);
    // 启动os
    OSStart();
}

fn task1(_args: *mut c_void) {
    loop {
        // 任务1
        #[cfg(feature = "defmt")]
        info!("---task1 begin---");
        OSTimeDly(100_000);
        #[cfg(feature = "defmt")]
        info!("---task1 end---");
        delay(SHORT_TIME);
    }
}
fn task2(_args: *mut c_void) {
    loop {
        // 任务2
        #[cfg(feature = "defmt")]
        info!("---task2 begin---");
        OSTimeDly(200_000);
        #[cfg(feature = "defmt")]
        info!("---task2 end---");
        delay(SHORT_TIME);
    }
}

async fn task3(_args: *mut c_void) {
    // 任务3
    loop {
        //
        #[cfg(feature = "defmt")]
        info!("---task3 begin---");
        Timer::after_secs(LONG_TIME as u64).await;
        // delay(LONG_TIME);
        #[cfg(feature = "defmt")]
        info!("---task3 end---");
        delay(SHORT_TIME);
    }
}
fn task4(_args: *mut c_void) {
    // 任务4
    #[cfg(feature = "defmt")]
    info!("---task4 begin---");
    // 任务3中涉及任务创建
    SyncOSTaskCreate(task1, 0 as *mut c_void, 0 as *mut usize, 14);
    delay(SHORT_TIME);
    #[cfg(feature = "defmt")]
    info!("---task4 end---");
    delay(MID_TIME);
}
