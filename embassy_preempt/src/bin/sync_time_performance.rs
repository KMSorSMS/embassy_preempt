#![no_main]
#![no_std]
#![feature(impl_trait_in_assoc_type)]
// this test is used to compare with embassy

use core::arch::asm;
use core::ffi::c_void;

use embassy_preempt::os_core::{OSInit, OSStart};
use embassy_preempt::os_task::SyncOSTaskCreate;
use embassy_preempt::os_time::OSTimeDly;
use embassy_preempt::pac::{gpio, GPIOA, RCC};
use embassy_preempt::port::bottom_driver::OSWaitBot;

#[cfg(feature = "alarm_test")]
use defmt::info;

const BLOCK_TIME: usize = 2;
const ONE_MS:u64 = 100;

// use embassy_preempt::{self as _};

#[cortex_m_rt::entry]
fn sync_time_performance() -> ! {
    // hardware init
    led_init();
    pin_init();
    // os初始化
    OSInit();
    SyncOSTaskCreate(test_task, 0 as *mut c_void, 0 as *mut usize, 10);
    SyncOSTaskCreate(task1, 0 as *mut c_void, 0 as *mut usize, 15);
    SyncOSTaskCreate(task2, 0 as *mut c_void, 0 as *mut usize, 14);
    SyncOSTaskCreate(task3, 0 as *mut c_void, 0 as *mut usize, 13);
    SyncOSTaskCreate(task4, 0 as *mut c_void, 0 as *mut usize, 12);
    SyncOSTaskCreate(task5, 0 as *mut c_void, 0 as *mut usize, 11);
    // 启动os
    OSStart();
}

// 主要测试任务
fn test_task(_args: *mut c_void) {
    loop {
        #[cfg(feature = "alarm_test")]
        info!("test task");
        // set the thread pin low, indicating that the thread time test is finished
        thread_pin_low();
        OSWaitBot();
        // set the interrupt pin low, indicating that the interrput and scheduling test is finished
        interrupt_pin_low();
        // set the thread pin high, indicating that the thread time test begins
        thread_pin_high();

        // delay 50ms
        // Timer::after_millis(50).await;
        OSTimeDly(50 * ONE_MS);
        thread_pin_low();
        // bottom::wait_for_rising_edge().await;
        OSWaitBot();
        interrupt_pin_low();
        thread_pin_high();
        // Timer::after_millis(50).await;
        OSTimeDly(50 * ONE_MS);
    }
}

/* 线程模式测试的TImer delay 修改为OSTime*/

// 用于模拟多任务执行环境
fn task1(_args: *mut c_void) {
    loop {
        #[cfg(feature = "alarm_test")]
        info!("the task1");
        // 将闪灯代码放入task1以免影响引脚设置和对Timer delay的测量
        led_on();
        // Timer::after_millis(5 * 100).await;
        OSTimeDly(500 * ONE_MS);
        led_off();
        // Timer::after_millis(5 * 100).await;
        OSTimeDly(500 * ONE_MS);
    }
}

// 用于模拟多任务执行环境
fn task2(_args: *mut c_void) {
    loop {
        #[cfg(feature = "alarm_test")]
        info!("the task2");
        delay(BLOCK_TIME);
        // Timer::after_millis(10).await;
        OSTimeDly(10 * ONE_MS);
    }
}

// 用于模拟多任务执行环境
fn task3(_args: *mut c_void) {
    loop {
        #[cfg(feature = "alarm_test")]
        info!("the task3");
        delay(BLOCK_TIME);
        // Timer::after_millis(20).await;
        OSTimeDly(20 * ONE_MS);        
    }
}

// 用于模拟多任务执行环境
fn task4(_args: *mut c_void) {
    loop {
        #[cfg(feature = "alarm_test")]
        info!("the task4");
        delay(BLOCK_TIME);
        // Timer::after_millis(30).await;
        OSTimeDly(30 * ONE_MS);
    }
}

// 用于模拟多任务执行环境
fn task5(_args: *mut c_void) {
    loop {
        #[cfg(feature = "alarm_test")]
        info!("the task5");
        delay(BLOCK_TIME);
        // Timer::after_millis(40).await;
        OSTimeDly(40 * ONE_MS);
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

/// TEST: thread pin and interrupt pin are used in the time_performance test
/// use the PA0 as the thread pin
/// use the PA1 as the interrupt pin
#[allow(dead_code)]
pub fn pin_init() {
    // enable the RCC
    RCC.ahb1enr().modify(|v| {
        v.set_gpioaen(true);
    });
    // set GPIO
    GPIOA.moder().modify(|v| {
        // set mode as output
        v.set_moder(0, gpio::vals::Moder::OUTPUT);
        v.set_moder(1, gpio::vals::Moder::OUTPUT);
    });
    GPIOA.otyper().modify(|v| {
        // set output type as push-pull
        v.set_ot(0, gpio::vals::Ot::PUSHPULL);
        v.set_ot(1, gpio::vals::Ot::PUSHPULL);
    });
    GPIOA.ospeedr().modify(|v| {
        // set output speed as high
        v.set_ospeedr(0, gpio::vals::Ospeedr::HIGHSPEED);
        v.set_ospeedr(1, gpio::vals::Ospeedr::HIGHSPEED);
    });
    GPIOA.pupdr().modify(|v| {
        // set pull-up/pull-down as no pull-up/pull-down
        v.set_pupdr(0, gpio::vals::Pupdr::FLOATING);
        v.set_pupdr(1, gpio::vals::Pupdr::FLOATING);
    });
    GPIOA.odr().modify(|v| {
        // set output as low
        v.set_odr(0, gpio::vals::Odr::LOW);
        v.set_odr(1, gpio::vals::Odr::LOW);
    });
}

/// set the thread pin high
#[allow(dead_code)]
#[inline]
pub fn thread_pin_high() {
    GPIOA.odr().modify(|v| {
        v.set_odr(0, gpio::vals::Odr::HIGH);
    });
}

/// set the thread pin low
#[allow(dead_code)]
#[inline]
pub fn thread_pin_low() {
    GPIOA.odr().modify(|v| {
        v.set_odr(0, gpio::vals::Odr::LOW);
    });
}

/// set the interrupt pin low
#[allow(dead_code)]
#[inline]
pub fn interrupt_pin_low() {
    GPIOA.odr().modify(|v| {
        v.set_odr(1, gpio::vals::Odr::LOW);
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
            in("r3") 800000/8,
        )
    }
}
