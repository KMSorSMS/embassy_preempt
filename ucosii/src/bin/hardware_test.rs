#![no_main]
#![no_std]
#![feature(impl_trait_in_assoc_type)]

// extern crate ucosii;
use defmt::info;
use ucosii::{app::led::{LED_Init, LED_OFF, LED_ON}, os_core::{OSInit, OSStart}, os_task::AsyncOSTaskCreate, os_time::timer::Timer};

#[cortex_m_rt::entry]
fn test_hardware() -> ! {
    // os初始化
    OSInit();
    // 为了测试硬件以及time driver的正确性，只创建1个任务以避免抢占
    AsyncOSTaskCreate(task1, 0 as *mut (), 0 as *mut usize, 10);
    // 启动os
    OSStart();
}

async fn task1(_args: *mut ()) {
    // init the led 
    LED_Init();
    loop{
        // led on
        LED_ON();
        info!("led on");
        // delay 5s
        Timer::after_secs(1).await;
        // led off
        LED_OFF();
        info!("led off");
        // delay 5s
        Timer::after_secs(1).await;
    }
}