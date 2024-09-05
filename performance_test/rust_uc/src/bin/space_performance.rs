#![no_main]
#![no_std]
#![feature(impl_trait_in_assoc_type)]
// this test is used to compare with embassy
use core::arch::asm;
use core::ffi::c_void;

use ucosii::app::led::Pin_Init;
// use ucosii::app::led::{LED_Init, Pin_Init, LED_OFF, LED_ON};
use ucosii::os_core::{OSInit, OSStart};
use ucosii::os_task::AsyncOSTaskCreate;
use ucosii::os_time::timer::Timer;
use ucosii::pac::{gpio, GPIOA, RCC};
const BLOCK_TIME: usize = 1;


#[cortex_m_rt::entry]
fn test_space_performance() -> ! {
    // hardware init
    led_init();
    Pin_Init();
    // os初始化
    OSInit();
    AsyncOSTaskCreate(test_task, 0 as *mut c_void, 0 as *mut usize, 10);
    AsyncOSTaskCreate(task1, 0 as *mut c_void, 0 as *mut usize, 11);
    AsyncOSTaskCreate(task2, 0 as *mut c_void, 0 as *mut usize, 12);
    AsyncOSTaskCreate(task3, 0 as *mut c_void, 0 as *mut usize, 13);
    AsyncOSTaskCreate(task4, 0 as *mut c_void, 0 as *mut usize, 14);
    AsyncOSTaskCreate(task5, 0 as *mut c_void, 0 as *mut usize, 15);
    AsyncOSTaskCreate(task6, 0 as *mut c_void, 0 as *mut usize, 16);
    AsyncOSTaskCreate(task7, 0 as *mut c_void, 0 as *mut usize, 17);
    AsyncOSTaskCreate(task8, 0 as *mut c_void, 0 as *mut usize, 18);
    AsyncOSTaskCreate(task9, 0 as *mut c_void, 0 as *mut usize, 19);
    AsyncOSTaskCreate(task10, 0 as *mut c_void, 0 as *mut usize, 20);
    AsyncOSTaskCreate(task11, 0 as *mut c_void, 0 as *mut usize, 21);
    AsyncOSTaskCreate(task12, 0 as *mut c_void, 0 as *mut usize, 22);
    AsyncOSTaskCreate(task13, 0 as *mut c_void, 0 as *mut usize, 23);
    AsyncOSTaskCreate(task14, 0 as *mut c_void, 0 as *mut usize, 24);
    AsyncOSTaskCreate(task15, 0 as *mut c_void, 0 as *mut usize, 25);
    AsyncOSTaskCreate(task16, 0 as *mut c_void, 0 as *mut usize, 26);
    AsyncOSTaskCreate(task17, 0 as *mut c_void, 0 as *mut usize, 27);
    AsyncOSTaskCreate(task18, 0 as *mut c_void, 0 as *mut usize, 28);
    AsyncOSTaskCreate(task19, 0 as *mut c_void, 0 as *mut usize, 29);
    AsyncOSTaskCreate(task20, 0 as *mut c_void, 0 as *mut usize, 30);
    // 启动os
    OSStart();
}

// 主要测试任务，在空间利用率测试中与其他任务无异
async fn test_task(_args: *mut c_void) {
    loop {
        // led on
        led_on();
        // delay 5s
        Timer::after_millis(5).await;
        // led off
        led_off();
        // delay 5s
        Timer::after_millis(5).await;
    }
}

// 用于模拟多任务执行环境，并且增加对比度
async fn task1(_args: *mut c_void) {
    loop {
        delay(BLOCK_TIME);
        Timer::after_millis(5).await;
        delay(BLOCK_TIME);
        Timer::after_millis(5).await;
    }
}

// 用于模拟多任务执行环境，并且增加对比度
async fn task2(_args: *mut c_void) {
    loop {
        delay(BLOCK_TIME);
        Timer::after_millis(5).await;
        delay(BLOCK_TIME);
        Timer::after_millis(5).await;
    }
}

// 用于模拟多任务执行环境，并且增加对比度
async fn task3(_args: *mut c_void) {
    loop {
        delay(BLOCK_TIME);
        Timer::after_millis(5).await;
        delay(BLOCK_TIME);
        Timer::after_millis(5).await;
    }
}

// 用于模拟多任务执行环境，并且增加对比度
async fn task4(_args: *mut c_void) {
    loop {
        delay(BLOCK_TIME);
        Timer::after_millis(5).await;
        delay(BLOCK_TIME);
        Timer::after_millis(5).await;
    }
}

// 用于模拟多任务执行环境，并且增加对比度
async fn task5(_args: *mut c_void) {
    loop {
        delay(BLOCK_TIME);
        Timer::after_millis(5).await;
        delay(BLOCK_TIME);
        Timer::after_millis(5).await;
    }
}

// 用于模拟多任务执行环境，并且增加对比度
async fn task6(_args: *mut c_void) {
    loop {
        delay(BLOCK_TIME);
        Timer::after_millis(5).await;
        delay(BLOCK_TIME);
        Timer::after_millis(5).await;
    }
}

// 用于模拟多任务执行环境，并且增加对比度
async fn task7(_args: *mut c_void) {
    loop {
        delay(BLOCK_TIME);
        Timer::after_millis(5).await;
        delay(BLOCK_TIME);
        Timer::after_millis(5).await;
    }
}

// 用于模拟多任务执行环境，并且增加对比度
async fn task8(_args: *mut c_void) {
    loop {
        delay(BLOCK_TIME);
        Timer::after_millis(5).await;
        delay(BLOCK_TIME);
        Timer::after_millis(5).await;
    }
}

// 用于模拟多任务执行环境，并且增加对比度
async fn task9(_args: *mut c_void) {
    loop {
        delay(BLOCK_TIME);
        Timer::after_millis(5).await;
        delay(BLOCK_TIME);
        Timer::after_millis(5).await;
    }
}

// 用于模拟多任务执行环境，并且增加对比度
async fn task10(_args: *mut c_void) {
    loop {
        delay(BLOCK_TIME);
        Timer::after_millis(5).await;
        delay(BLOCK_TIME);
        Timer::after_millis(5).await;
    }
}

// 用于模拟多任务执行环境，并且增加对比度
async fn task11(_args: *mut c_void) {
    loop {
        delay(BLOCK_TIME);
        Timer::after_millis(5).await;
        delay(BLOCK_TIME);
        Timer::after_millis(5).await;
    }
}

// 用于模拟多任务执行环境，并且增加对比度
async fn task12(_args: *mut c_void) {
    loop {
        delay(BLOCK_TIME);
        Timer::after_millis(5).await;
        delay(BLOCK_TIME);
        Timer::after_millis(5).await;
    }
}

// 用于模拟多任务执行环境，并且增加对比度
async fn task13(_args: *mut c_void) {
    loop {
        delay(BLOCK_TIME);
        Timer::after_millis(5).await;
        delay(BLOCK_TIME);
        Timer::after_millis(5).await;
    }
}

// 用于模拟多任务执行环境，并且增加对比度
async fn task14(_args: *mut c_void) {
    loop {
        delay(BLOCK_TIME);
        Timer::after_millis(5).await;
        delay(BLOCK_TIME);
        Timer::after_millis(5).await;
    }
}

// 用于模拟多任务执行环境，并且增加对比度
async fn task15(_args: *mut c_void) {
    loop {
        delay(BLOCK_TIME);
        Timer::after_millis(5).await;
        delay(BLOCK_TIME);
        Timer::after_millis(5).await;
    }
}

// 用于模拟多任务执行环境，并且增加对比度
async fn task16(_args: *mut c_void) {
    loop {
        delay(BLOCK_TIME);
        Timer::after_millis(5).await;
        delay(BLOCK_TIME);
        Timer::after_millis(5).await;
    }
}

// 用于模拟多任务执行环境，并且增加对比度
async fn task17(_args: *mut c_void) {
    loop {
        delay(BLOCK_TIME);
        Timer::after_millis(5).await;
        delay(BLOCK_TIME);
        Timer::after_millis(5).await;
    }
}

// 用于模拟多任务执行环境，并且增加对比度
async fn task18(_args: *mut c_void) {
    loop {
        delay(BLOCK_TIME);
        Timer::after_millis(5).await;
        delay(BLOCK_TIME);
        Timer::after_millis(5).await;
    }
}

// 用于模拟多任务执行环境，并且增加对比度
async fn task19(_args: *mut c_void) {
    loop {
        delay(BLOCK_TIME);
        Timer::after_millis(5).await;
        delay(BLOCK_TIME);
        Timer::after_millis(5).await;
    }
}

// 用于模拟多任务执行环境，并且增加对比度
async fn task20(_args: *mut c_void) {
    loop {
        delay(BLOCK_TIME);
        Timer::after_millis(5).await;
        delay(BLOCK_TIME);
        Timer::after_millis(5).await;
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

/// block delay
#[inline]
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
            in("r3") 8000000/8,
        )
    }
}
