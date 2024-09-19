#![no_main]
#![no_std]
#![feature(impl_trait_in_assoc_type)]
// this test is used to compare with embassy

use core::arch::asm;
use core::ffi::c_void;

#[cfg(feature = "alarm_test")]
use defmt::trace;
use embassy_preempt::app::led::{interrupt_pin_low, thread_pin_high, thread_pin_low, Pin_Init};
use embassy_preempt::os_core::{OSInit, OSStart};
use embassy_preempt::os_task::AsyncOSTaskCreate;
use embassy_preempt::os_time::timer::Timer;
use embassy_preempt::pac::{gpio, GPIOA, RCC};
use embassy_preempt::port::bottom_driver::Bottom::bottom;

const BLOCK_TIME: usize = 1;

// use embassy_preempt::{self as _};

#[cortex_m_rt::entry]
fn test_time_performance() -> ! {
    // hardware init
    led_init();
    Pin_Init();
    // os初始化
    OSInit();
    AsyncOSTaskCreate(test_task, 0 as *mut c_void, 0 as *mut usize, 10);
    AsyncOSTaskCreate(task1, 0 as *mut c_void, 0 as *mut usize, 15);
    AsyncOSTaskCreate(task2, 0 as *mut c_void, 0 as *mut usize, 14);
    AsyncOSTaskCreate(task3, 0 as *mut c_void, 0 as *mut usize, 13);
    AsyncOSTaskCreate(task4, 0 as *mut c_void, 0 as *mut usize, 12);
    AsyncOSTaskCreate(task5, 0 as *mut c_void, 0 as *mut usize, 11);
    // 启动os
    OSStart();
}

// 主要测试任务
async fn test_task(_args: *mut c_void) {
    loop {
        // set the thread pin low, indicating that the thread time test is finished
        thread_pin_low();
        bottom::wait_for_rising_edge().await;
        // set the interrupt pin low, indicating that the interrput and scheduling test is finished
        interrupt_pin_low();
        // set the thread pin high, indicating that the thread time test begins
        thread_pin_high();

        // delay 5s
        Timer::after_millis(50).await;
        thread_pin_low();
        bottom::wait_for_rising_edge().await;
        interrupt_pin_low();
        thread_pin_high();
        Timer::after_millis(50).await;
    }
}

// 用于模拟多任务执行环境
async fn task1(_args: *mut c_void) {
    loop {
        // 将闪灯代码放入task1以免影响引脚设置和对Timer delay的测量
        led_on();
        Timer::after_millis(5 * 1000).await;
        led_off();
        #[cfg(feature = "alarm_test")]
        trace!("the task1");
        Timer::after_millis(5 * 1000).await;
    }
}

// 用于模拟多任务执行环境
async fn task2(_args: *mut c_void) {
    loop {
        delay(BLOCK_TIME);
        // Timer::after_millis(10).await;
        #[cfg(feature = "alarm_test")]
        trace!("the task2");
        delay(BLOCK_TIME);
        Timer::after_millis(5).await;
    }
}

// 用于模拟多任务执行环境
async fn task3(_args: *mut c_void) {
    loop {
        delay(BLOCK_TIME);
        // Timer::after_millis(20).await;
        // #[cfg(feature = "alarm_test")]
        // trace!("the task3");
        delay(BLOCK_TIME);
        #[cfg(feature = "alarm_test")]
        trace!("the task3");
        Timer::after_millis(20).await;
    }
}

// 用于模拟多任务执行环境
async fn task4(_args: *mut c_void) {
    loop {
        delay(BLOCK_TIME);
        // Timer::after_millis(80).await;
        #[cfg(feature = "alarm_test")]
        trace!("the task4");
        delay(BLOCK_TIME);
        Timer::after_millis(8).await;
    }
}

// 用于模拟多任务执行环境
async fn task5(_args: *mut c_void) {
    loop {
        delay(BLOCK_TIME);
        // Timer::after_millis(300).await;
        #[cfg(feature = "alarm_test")]
        trace!("the task5");
        delay(BLOCK_TIME);
        Timer::after_millis(10).await;
    }
}

/// init the LED
#[allow(dead_code)]
pub fn led_init() {
    // enable the RCC
    RCC.ahb1enr().modify(|v| {
        v.set_gpioaen(true);
    });
    // set GPIO
    GPIOA.moder().modify(|v| {
        // set mode as output
        v.set_moder(5, gpio::vals::Moder::OUTPUT);
    });
    GPIOA.otyper().modify(|v| {
        // set output type as push-pull
        v.set_ot(5, gpio::vals::Ot::PUSHPULL);
    });
    GPIOA.ospeedr().modify(|v| {
        // set output speed as high
        v.set_ospeedr(5, gpio::vals::Ospeedr::HIGHSPEED);
    });
    GPIOA.pupdr().modify(|v| {
        // set pull-up/pull-down as no pull-up/pull-down
        v.set_pupdr(5, gpio::vals::Pupdr::FLOATING);
    });
    GPIOA.odr().modify(|v| {
        // set output as high
        v.set_odr(5, gpio::vals::Odr::HIGH);
    });
}

/// turn on the LED
#[allow(dead_code)]
#[inline]
pub fn led_on() {
    GPIOA.odr().modify(|v| {
        v.set_odr(5, gpio::vals::Odr::HIGH);
    });
}

/// turn off the LED
#[allow(dead_code)]
#[inline]
pub fn led_off() {
    GPIOA.odr().modify(|v| {
        v.set_odr(5, gpio::vals::Odr::LOW);
    });
}

#[inline(never)]
pub fn delay(time: usize) {
    // 延时函数,time的单位约为0.5s，使用汇编编写从而不会被优化
    unsafe {
        asm!(
            // 先来个循环（总共是两层循环，内层循环次数8000000）
            "mov r0, #0",
            "1:",
            // 内层循环
            "mov r1, #0",
            "2:",
            "add r1, r1, #1",
            "cmp r1, r3",
            "blt 2b",
            // 外层循环
            "add r0, r0, #1",
            "cmp r0, r2",
            "blt 1b",
            in("r2") time,
            in("r3") 1000000/8,
        )
    }
}
