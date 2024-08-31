#![no_main]
#![no_std]
#![feature(impl_trait_in_assoc_type)]

use core::ffi::c_void;

// extern crate ucosii;
#[cfg(feature = "defmt")]
use defmt::info;
use ucosii::app::led::{LED_Init, LED_OFF, LED_ON};
use ucosii::os_core::{OSInit, OSStart};
use ucosii::os_task::AsyncOSTaskCreate;
// use ucosii::os_time::timer::Timer;
use ucosii::port::bottom_driver::Bottom::bottom;
// use ucosii::{self as _};

#[cortex_m_rt::entry]
fn test_hardware() -> ! {
    // os初始化
    OSInit();
    // init the led
    LED_Init();
    // 为了测试硬件以及time driver的正确性，只创建1个任务以避免抢占
    AsyncOSTaskCreate(task1, 0 as *mut c_void, 0 as *mut usize, 10);
    // 启动os
    OSStart();
}

async fn task1(_args: *mut c_void) {
    loop {
        // led on
        LED_ON();
        #[cfg(feature = "defmt")]
        info!("led on");
        bottom::wait_for_rising_edge().await;
        // led off
        LED_OFF();
        #[cfg(feature = "defmt")]
        info!("led off");
        // delay(1);
        // delay 5s
        bottom::wait_for_rising_edge().await;
    }
}