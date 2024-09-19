#![no_main]
#![no_std]
#![feature(impl_trait_in_assoc_type)]

use core::ffi::c_void;

// extern crate embassy_preempt;
#[cfg(feature = "alarm_test")]
use defmt::info;
use embassy_preempt::app::led::{LED_Init, LED_OFF, LED_ON};
use embassy_preempt::os_core::{OSInit, OSStart};
use embassy_preempt::os_task::SyncOSTaskCreate;
use embassy_preempt::port::bottom_driver::OSWaitBot;
// use embassy_preempt::os_time::timer::Timer;
// use embassy_preempt::port::bottom_driver::Bottom::bottom;
// use embassy_preempt::{self as _};

#[cortex_m_rt::entry]
fn test_hardware() -> ! {
    // os初始化
    OSInit();
    // init the led
    LED_Init();
    // 为了测试硬件以及time driver的正确性，只创建1个任务以避免抢占
    SyncOSTaskCreate(task1, 0 as *mut c_void, 0 as *mut usize, 10);
    // 启动os
    OSStart();
}

fn task1(_args: *mut c_void) {
    // led on
    LED_ON();
    loop {
        // led on
        LED_ON();
        #[cfg(feature = "alarm_test")]
        info!("led on");
        // bottom::wait_for_rising_edge().await;
        OSWaitBot();
        // led off
        LED_OFF();
        #[cfg(feature = "alarm_test")]
        info!("led off");
        // delay(1);
        // delay 5s
        // bottom::wait_for_rising_edge().await;
        OSWaitBot();
    }
}