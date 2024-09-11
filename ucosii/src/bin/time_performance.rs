#![no_main]
#![no_std]
#![feature(impl_trait_in_assoc_type)]
// this test is used to compare with embassy

use core::arch::asm;
use core::ffi::c_void;

#[cfg(feature = "alarm_test")]
use defmt::{info,trace};
use ucosii::os_core::{OSInit, OSStart};
use ucosii::os_task::AsyncOSTaskCreate;
use ucosii::os_time::timer::Timer;
use ucosii::pac::{gpio, GPIOA, RCC};
use ucosii::port::bottom_driver::Bottom::bottom;

const BLOCK_TIME: usize =1;

// use ucosii::{self as _};

#[cortex_m_rt::entry]
fn test_time_performance() -> ! {
    // hardware init
    led_init();
    pin_init();
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
        #[cfg(feature = "alarm_test")]
        trace!("the task1");
        Timer::after_millis(5 * 1000).await;
        led_off();
        Timer::after_millis(5 * 1000).await;
    }
}

// 用于模拟多任务执行环境
async fn task2(_args: *mut c_void) {
    loop {
        task_pin_low(2);
        delay(BLOCK_TIME);
        // Timer::after_millis(10).await;
        #[cfg(feature = "alarm_test")]
        trace!("the task2");
        delay(BLOCK_TIME);
        task_pin_high(2);
        Timer::after_millis(1000).await;
    }
}

// 用于模拟多任务执行环境
async fn task3(_args: *mut c_void) {
    loop {
        task_pin_low(3);
        delay(BLOCK_TIME);
        // Timer::after_millis(20).await;
        #[cfg(feature = "alarm_test")]
        trace!("the task3");
        delay(BLOCK_TIME);
        task_pin_high(3);
        Timer::after_millis(200).await;
    }
}

// 用于模拟多任务执行环境
async fn task4(_args: *mut c_void) {
    loop {
        task_pin_low(4);
        delay(BLOCK_TIME);
        // Timer::after_millis(80).await;
        #[cfg(feature = "alarm_test")]
        trace!("the task4");
        delay(BLOCK_TIME);
        task_pin_high(4);
        Timer::after_millis(800).await;
    }
}

// 用于模拟多任务执行环境
async fn task5(_args: *mut c_void) {
    loop {
        task_pin_low(5);
        delay(BLOCK_TIME);
        // Timer::after_millis(300).await;
        #[cfg(feature = "alarm_test")]
        trace!("the task5");
        delay(BLOCK_TIME);
        task_pin_high(5);
        Timer::after_millis(300).await;
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
        v.set_moder(4, gpio::vals::Moder::OUTPUT);
        v.set_moder(6, gpio::vals::Moder::OUTPUT);
        v.set_moder(7, gpio::vals::Moder::OUTPUT);
        v.set_moder(8, gpio::vals::Moder::OUTPUT);
    });
    GPIOA.otyper().modify(|v| {
        // set output type as push-pull
        v.set_ot(0, gpio::vals::Ot::PUSHPULL);
        v.set_ot(1, gpio::vals::Ot::PUSHPULL);
        v.set_ot(4, gpio::vals::Ot::PUSHPULL);
        v.set_ot(6, gpio::vals::Ot::PUSHPULL);
        v.set_ot(7, gpio::vals::Ot::PUSHPULL);
        v.set_ot(8, gpio::vals::Ot::PUSHPULL);
    });
    GPIOA.ospeedr().modify(|v| {
        // set output speed as high
        v.set_ospeedr(0, gpio::vals::Ospeedr::HIGHSPEED);
        v.set_ospeedr(1, gpio::vals::Ospeedr::HIGHSPEED);
        v.set_ospeedr(4, gpio::vals::Ospeedr::HIGHSPEED);
        v.set_ospeedr(6, gpio::vals::Ospeedr::HIGHSPEED);
        v.set_ospeedr(7, gpio::vals::Ospeedr::HIGHSPEED);
        v.set_ospeedr(8, gpio::vals::Ospeedr::HIGHSPEED);
    });
    GPIOA.pupdr().modify(|v| {
        // set pull-up/pull-down as no pull-up/pull-down
        v.set_pupdr(0, gpio::vals::Pupdr::FLOATING);
        v.set_pupdr(1, gpio::vals::Pupdr::FLOATING);
        v.set_pupdr(4, gpio::vals::Pupdr::FLOATING);
        v.set_pupdr(6, gpio::vals::Pupdr::FLOATING);
        v.set_pupdr(7, gpio::vals::Pupdr::FLOATING);
        v.set_pupdr(8, gpio::vals::Pupdr::FLOATING);
    });
    GPIOA.odr().modify(|v| {
        // set output as low
        v.set_odr(0, gpio::vals::Odr::LOW);
        v.set_odr(1, gpio::vals::Odr::LOW);
        v.set_odr(4, gpio::vals::Odr::LOW);
        v.set_odr(6, gpio::vals::Odr::LOW);
        v.set_odr(7, gpio::vals::Odr::LOW);
        v.set_odr(8, gpio::vals::Odr::LOW);
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

/// set the task() pin high
#[allow(dead_code)]
#[inline]
pub fn task_pin_high(task: usize) {
    critical_section::with(|_| {
        let task = match task {
            2 => 4,
            3 => 6,
            4 => 7,
            5 => 8,
            _ => 0,
        };
        GPIOA.odr().modify(|v| {
            v.set_odr(task, gpio::vals::Odr::HIGH);
        });
    });
}

/// set the task() pin low
#[allow(dead_code)]
#[inline]
pub fn task_pin_low(task: usize) {
    critical_section::with(|_| {
        let task = match task {
            2 => 4,
            3 => 6,
            4 => 7,
            5 => 8,
            _ => 0,
        };
        GPIOA.odr().modify(|v| {
            v.set_odr(task, gpio::vals::Odr::LOW);
        });
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
            in("r3") 4000000/8,
        )
    }
}
