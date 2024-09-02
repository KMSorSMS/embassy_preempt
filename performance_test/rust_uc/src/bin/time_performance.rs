#![no_main]
#![no_std]
#![feature(impl_trait_in_assoc_type)]
// this test is used to compare with embassy

use core::ffi::c_void;

use ucosii::app::led::{interrupt_pin_low, thread_pin_high, thread_pin_low, LED_Init, Pin_Init, LED_OFF, LED_ON};
use ucosii::os_core::{OSInit, OSStart};
use ucosii::os_task::AsyncOSTaskCreate;
use ucosii::os_time::timer::Timer;
use ucosii::port::bottom_driver::Bottom::bottom;
// use ucosii::{self as _};

#[cortex_m_rt::entry]
fn test_time_performance() -> ! {
    // hardware init
    LED_Init();
    Pin_Init();
    // os初始化
    OSInit();
    AsyncOSTaskCreate(test_task,0 as *mut c_void,0 as *mut usize,10);
    AsyncOSTaskCreate(task1, 0 as *mut c_void, 0 as *mut usize, 11);
    AsyncOSTaskCreate(task2, 0 as *mut c_void, 0 as *mut usize, 12);
    AsyncOSTaskCreate(task3, 0 as *mut c_void, 0 as *mut usize, 13);
    AsyncOSTaskCreate(task4, 0 as *mut c_void, 0 as *mut usize, 14);
    AsyncOSTaskCreate(task5, 0 as *mut c_void, 0 as *mut usize, 15);
    // 启动os
    OSStart();
}

// 主要测试任务
async fn test_task(_args: *mut c_void) {
    loop {
        bottom::wait_for_rising_edge().await;
        // set the interrupt pin low, indicating that the interrput and scheduling test is finished
        interrupt_pin_low();
        // set the thread pin high, indicating that the thread time test begins
        thread_pin_high();
        
        // delay 5s
        Timer::after_secs(5).await;

        // set the thread pin low, indicating that the thread time test is finished
        thread_pin_low();
    }
}

// 用于模拟多任务执行环境
async fn task1(_args: *mut c_void) {
    loop {
        // led on
        LED_ON();
        // delay 5s
        Timer::after_secs(5).await;
        // led off
        LED_OFF();
        // delay 5s
        Timer::after_secs(5).await;
    }
}

// 用于模拟多任务执行环境
async fn task2(_args: *mut c_void) {
    loop {
        // led on
        LED_ON();
        // delay 5s
        Timer::after_secs(5).await;
        // led off
        LED_OFF();
        // delay 5s
        Timer::after_secs(5).await;
    }
}

// 用于模拟多任务执行环境
async fn task3(_args: *mut c_void) {
    loop {
        // led on
        LED_ON();
        // delay 5s
        Timer::after_secs(5).await;
        // led off
        LED_OFF();
        // delay 5s
        Timer::after_secs(5).await;
    }
}

// 用于模拟多任务执行环境
async fn task4(_args: *mut c_void) {
    loop {
        // led on
        LED_ON();
        // delay 5s
        Timer::after_secs(5).await;
        // led off
        LED_OFF();
        // delay 5s
        Timer::after_secs(5).await;
    }
}

// 用于模拟多任务执行环境
async fn task5(_args: *mut c_void) {
    loop {
        // led on
        LED_ON();
        // delay 5s
        Timer::after_secs(5).await;
        // led off
        LED_OFF();
        // delay 5s
        Timer::after_secs(5).await;
    }
}