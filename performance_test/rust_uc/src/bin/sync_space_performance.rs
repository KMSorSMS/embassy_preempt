#![no_main]
#![no_std]
#![feature(impl_trait_in_assoc_type)]
use core::arch::asm;
use core::ffi::c_void;

use ucosii::app::led::Pin_Init;
use ucosii::os_core::{OSInit, OSStart};
use ucosii::os_task::SyncOSTaskCreate;
use ucosii::os_time::OSTimeDly;
use ucosii::pac::{gpio, GPIOA, RCC};
const DELAY_TICK: u64 = 5 * 100;
const BLOCK_TIME: usize = 1;

#[cortex_m_rt::entry]
fn test_space_performance() -> ! {
    // hardware init
    led_init();
    Pin_Init();
    // os初始化
    OSInit();
    SyncOSTaskCreate(test_task, 0 as *mut c_void, 0 as *mut usize, 10);
    SyncOSTaskCreate(task1, 0 as *mut c_void, 0 as *mut usize, 11);
    SyncOSTaskCreate(task2, 0 as *mut c_void, 0 as *mut usize, 12);
    SyncOSTaskCreate(task3, 0 as *mut c_void, 0 as *mut usize, 13);
    SyncOSTaskCreate(task4, 0 as *mut c_void, 0 as *mut usize, 14);
    SyncOSTaskCreate(task5, 0 as *mut c_void, 0 as *mut usize, 15);
    SyncOSTaskCreate(task6, 0 as *mut c_void, 0 as *mut usize, 16);
    SyncOSTaskCreate(task7, 0 as *mut c_void, 0 as *mut usize, 17);
    SyncOSTaskCreate(task8, 0 as *mut c_void, 0 as *mut usize, 18);
    SyncOSTaskCreate(task9, 0 as *mut c_void, 0 as *mut usize, 19);
    SyncOSTaskCreate(task10, 0 as *mut c_void, 0 as *mut usize, 20);
    SyncOSTaskCreate(task11, 0 as *mut c_void, 0 as *mut usize, 21);
    SyncOSTaskCreate(task12, 0 as *mut c_void, 0 as *mut usize, 22);
    SyncOSTaskCreate(task13, 0 as *mut c_void, 0 as *mut usize, 23);
    SyncOSTaskCreate(task14, 0 as *mut c_void, 0 as *mut usize, 24);
    SyncOSTaskCreate(task15, 0 as *mut c_void, 0 as *mut usize, 25);
    SyncOSTaskCreate(task16, 0 as *mut c_void, 0 as *mut usize, 26);
    SyncOSTaskCreate(task17, 0 as *mut c_void, 0 as *mut usize, 27);
    SyncOSTaskCreate(task18, 0 as *mut c_void, 0 as *mut usize, 28);
    SyncOSTaskCreate(task19, 0 as *mut c_void, 0 as *mut usize, 29);
    SyncOSTaskCreate(task20, 0 as *mut c_void, 0 as *mut usize, 30);
    // 启动os
    OSStart();
}

fn test_task(_args: *mut c_void) {
    loop {
        // led on
        led_on();
        // delay 5s
        OSTimeDly(DELAY_TICK);
        // led off
        led_off();
        // delay 5s
        OSTimeDly(DELAY_TICK);
    }
}

// 用于模拟多任务执行环境，并且增加对比度
fn task1(_args: *mut c_void) {
    loop {
        delay(BLOCK_TIME);
        OSTimeDly(DELAY_TICK);
        delay(BLOCK_TIME);
        OSTimeDly(DELAY_TICK);
    }
}

// 用于模拟多任务执行环境，并且增加对比度
fn task2(_args: *mut c_void) {
    loop {
        delay(BLOCK_TIME);
        OSTimeDly(DELAY_TICK);
        delay(BLOCK_TIME);
        OSTimeDly(DELAY_TICK);
    }
}

// 用于模拟多任务执行环境，并且增加对比度
fn task3(_args: *mut c_void) {
    loop {
        delay(BLOCK_TIME);
        OSTimeDly(DELAY_TICK);
        delay(BLOCK_TIME);
        OSTimeDly(DELAY_TICK);
    }
}

// 用于模拟多任务执行环境，并且增加对比度
fn task4(_args: *mut c_void) {
    loop {
        delay(BLOCK_TIME);
        OSTimeDly(DELAY_TICK);
        delay(BLOCK_TIME);
        OSTimeDly(DELAY_TICK);
    }
}

// 用于模拟多任务执行环境，并且增加对比度
fn task5(_args: *mut c_void) {
    loop {
        delay(BLOCK_TIME);
        OSTimeDly(DELAY_TICK);
        delay(BLOCK_TIME);
        OSTimeDly(DELAY_TICK);
    }
}

// 用于模拟多任务执行环境，并且增加对比度
fn task6(_args: *mut c_void) {
    loop {
        delay(BLOCK_TIME);
        OSTimeDly(DELAY_TICK);
        delay(BLOCK_TIME);
        OSTimeDly(DELAY_TICK);
    }
}

// 用于模拟多任务执行环境，并且增加对比度
fn task7(_args: *mut c_void) {
    loop {
        delay(BLOCK_TIME);
        OSTimeDly(DELAY_TICK);
        delay(BLOCK_TIME);
        OSTimeDly(DELAY_TICK);
    }
}

// 用于模拟多任务执行环境，并且增加对比度
fn task8(_args: *mut c_void) {
    loop {
        delay(BLOCK_TIME);
        OSTimeDly(DELAY_TICK);
        delay(BLOCK_TIME);
        OSTimeDly(DELAY_TICK);
    }
}

// 用于模拟多任务执行环境，并且增加对比度
fn task9(_args: *mut c_void) {
    loop {
        delay(BLOCK_TIME);
        OSTimeDly(DELAY_TICK);
        delay(BLOCK_TIME);
        OSTimeDly(DELAY_TICK);
    }
}

// 用于模拟多任务执行环境，并且增加对比度
fn task10(_args: *mut c_void) {
    loop {
        delay(BLOCK_TIME);
        OSTimeDly(DELAY_TICK);
        delay(BLOCK_TIME);
        OSTimeDly(DELAY_TICK);
    }
}

// 用于模拟多任务执行环境，并且增加对比度
fn task11(_args: *mut c_void) {
    loop {
        delay(BLOCK_TIME);
        OSTimeDly(DELAY_TICK);
        delay(BLOCK_TIME);
        OSTimeDly(DELAY_TICK);
    }
}

// 用于模拟多任务执行环境，并且增加对比度
fn task12(_args: *mut c_void) {
    loop {
        delay(BLOCK_TIME);
        OSTimeDly(DELAY_TICK);
        delay(BLOCK_TIME);
        OSTimeDly(DELAY_TICK);
    }
}

// 用于模拟多任务执行环境，并且增加对比度
fn task13(_args: *mut c_void) {
    loop {
        delay(BLOCK_TIME);
        OSTimeDly(DELAY_TICK);
        delay(BLOCK_TIME);
        OSTimeDly(DELAY_TICK);
    }
}

// 用于模拟多任务执行环境，并且增加对比度
fn task14(_args: *mut c_void) {
    loop {
        delay(BLOCK_TIME);
        OSTimeDly(DELAY_TICK);
        delay(BLOCK_TIME);
        OSTimeDly(DELAY_TICK);
    }
}

// 用于模拟多任务执行环境，并且增加对比度
fn task15(_args: *mut c_void) {
    loop {
        delay(BLOCK_TIME);
        OSTimeDly(DELAY_TICK);
        delay(BLOCK_TIME);
        OSTimeDly(DELAY_TICK);
    }
}

// 用于模拟多任务执行环境，并且增加对比度
fn task16(_args: *mut c_void) {
    loop {
        delay(BLOCK_TIME);
        OSTimeDly(DELAY_TICK);
        delay(BLOCK_TIME);
        OSTimeDly(DELAY_TICK);
    }
}

// 用于模拟多任务执行环境，并且增加对比度
fn task17(_args: *mut c_void) {
    loop {
        delay(BLOCK_TIME);
        OSTimeDly(DELAY_TICK);
        delay(BLOCK_TIME);
        OSTimeDly(DELAY_TICK);
    }
}

// 用于模拟多任务执行环境，并且增加对比度
fn task18(_args: *mut c_void) {
    loop {
        delay(BLOCK_TIME);
        OSTimeDly(DELAY_TICK);
        delay(BLOCK_TIME);
        OSTimeDly(DELAY_TICK);
    }
}

// 用于模拟多任务执行环境，并且增加对比度
fn task19(_args: *mut c_void) {
    loop {
        delay(BLOCK_TIME);
        OSTimeDly(DELAY_TICK);
        delay(BLOCK_TIME);
        OSTimeDly(DELAY_TICK);
    }
}

// 用于模拟多任务执行环境，并且增加对比度
fn task20(_args: *mut c_void) {
    loop {
        delay(BLOCK_TIME);
        OSTimeDly(DELAY_TICK);
        delay(BLOCK_TIME);
        OSTimeDly(DELAY_TICK);
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
