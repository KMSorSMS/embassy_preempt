#![no_std]
#![no_main]

use core::arch::asm;

use embassy_executor::Spawner;
use embassy_stm32::{gpio::{Level, Output, Speed}, rcc::Pll};
use embassy_test as _;
use embassy_time::Timer;
use stm32_metapac::rcc;
const BLOCK_TIME: usize = 1;

// 主要测试任务
#[embassy_executor::main]
async fn main(spawner: Spawner) {
    // 硬件初始化
    let hse = Some(embassy_stm32::rcc::Hse {
        freq: embassy_stm32::time::Hertz(8_000_000),
        mode: embassy_stm32::rcc::HseMode::Oscillator,
    });
    let pll = Some(Pll {
        prediv: rcc::vals::Pllm::DIV4,
        mul: rcc::vals::Plln::MUL84,
        divp: Some(rcc::vals::Pllp::DIV2),
        divq: Some(rcc::vals::Pllq::DIV4),
        divr: None,
    });
    let mut rcc = embassy_stm32::rcc::Config::default();
    // config the default mannually, its dull
    rcc.hsi = false;
    rcc.hse = hse;
    rcc.sys = rcc::vals::Sw::PLL1_P;
    rcc.pll_src = rcc::vals::Pllsrc::HSE;
    rcc.pll = pll;
    rcc.ahb_pre = rcc::vals::Hpre::DIV1;
    rcc.apb1_pre = rcc::vals::Ppre::DIV2;
    rcc.apb2_pre = rcc::vals::Ppre::DIV2;

    let mut config = embassy_stm32::Config::default();
    config.rcc = rcc;
    let p = embassy_stm32::init(config);

    // 初始化LED
    let led = Output::new(p.PA5, Level::High, Speed::High);

    // 初始化按键，以及对应中断。在空间利用率测试中，按键没有作用，所以不再进行初始化
    // let button = ExtiInput::new(p.PC13, p.EXTI13,Pull::Down);

    // 创建任务
    spawner.spawn(test_task(led)).unwrap();
    spawner.spawn(task1()).unwrap();
    spawner.spawn(task2()).unwrap();
    spawner.spawn(task3()).unwrap();
    spawner.spawn(task4()).unwrap();
    spawner.spawn(task5()).unwrap();
    spawner.spawn(task6()).unwrap();
    spawner.spawn(task7()).unwrap();
    spawner.spawn(task8()).unwrap();
    spawner.spawn(task9()).unwrap();
    spawner.spawn(task10()).unwrap();
    spawner.spawn(task11()).unwrap();
    spawner.spawn(task12()).unwrap();
    spawner.spawn(task13()).unwrap();
    spawner.spawn(task14()).unwrap();
    spawner.spawn(task15()).unwrap();
    spawner.spawn(task16()).unwrap();
    spawner.spawn(task17()).unwrap();
    spawner.spawn(task18()).unwrap();
    spawner.spawn(task19()).unwrap();
    spawner.spawn(task20()).unwrap();
}

// 用于模拟多任务执行环境，并且增加对比度
#[embassy_executor::task]
async fn test_task(mut led: Output<'static>) {
    loop {
        // 将闪灯代码放入task1以免影响引脚设置和对Timer delay的测量
        led.set_high();
        Timer::after_millis(5).await;
        led.set_low();
        Timer::after_millis(5).await;
    }
}

// 用于模拟多任务执行环境，并且增加对比度
#[embassy_executor::task]
async fn task1() {
    loop {
        // 插入阻塞等待(阻塞3s)
        delay(BLOCK_TIME);
        Timer::after_millis(5).await;
        delay(BLOCK_TIME);
        Timer::after_millis(5).await;
    }
}

// 用于模拟多任务执行环境，并且增加对比度
#[embassy_executor::task]
async fn task2() {
    loop {
        delay(BLOCK_TIME);
        Timer::after_millis(5).await;
        delay(BLOCK_TIME);
        Timer::after_millis(5).await;
    }
}

// 用于模拟多任务执行环境，并且增加对比度
#[embassy_executor::task]
async fn task3() {
    loop {
        delay(BLOCK_TIME);
        Timer::after_millis(5).await;
        delay(BLOCK_TIME);
        Timer::after_millis(5).await;
    }
}

// 用于模拟多任务执行环境，并且增加对比度
#[embassy_executor::task]
async fn task4() {
    loop {
        delay(BLOCK_TIME);
        Timer::after_millis(5).await;
        delay(BLOCK_TIME);
        Timer::after_millis(5).await;
    }
}

// 用于模拟多任务执行环境，并且增加对比度
#[embassy_executor::task]
async fn task5() {
    loop {
        delay(BLOCK_TIME);
        Timer::after_millis(5).await;
        delay(BLOCK_TIME);
        Timer::after_millis(5).await;
    }
}

// 用于模拟多任务执行环境，并且增加对比度
#[embassy_executor::task]
async fn task6() {
    loop {
        delay(BLOCK_TIME);
        Timer::after_millis(5).await;
        delay(BLOCK_TIME);
        Timer::after_millis(5).await;
    }
}

// 用于模拟多任务执行环境，并且增加对比度
#[embassy_executor::task]
async fn task7() {
    loop {
        delay(BLOCK_TIME);
        Timer::after_millis(5).await;
        delay(BLOCK_TIME);
        Timer::after_millis(5).await;
    }
}

// 用于模拟多任务执行环境，并且增加对比度
#[embassy_executor::task]
async fn task8() {
    loop {
        delay(BLOCK_TIME);
        Timer::after_millis(5).await;
        delay(BLOCK_TIME);
        Timer::after_millis(5).await;
    }
}

// 用于模拟多任务执行环境，并且增加对比度
#[embassy_executor::task]
async fn task9() {
    loop {
        delay(BLOCK_TIME);
        Timer::after_millis(5).await;
        delay(BLOCK_TIME);
        Timer::after_millis(5).await;
    }
}

// 用于模拟多任务执行环境，并且增加对比度
#[embassy_executor::task]
async fn task10() {
    loop {
        delay(BLOCK_TIME);
        Timer::after_millis(5).await;
        delay(BLOCK_TIME);
        Timer::after_millis(5).await;
    }
}

// 用于模拟多任务执行环境，并且增加对比度
#[embassy_executor::task]
async fn task11() {
    loop {
        delay(BLOCK_TIME);
        Timer::after_millis(5).await;
        delay(BLOCK_TIME);
        Timer::after_millis(5).await;
    }
}

// 用于模拟多任务执行环境，并且增加对比度
#[embassy_executor::task]
async fn task12() {
    loop {
        delay(BLOCK_TIME);
        Timer::after_millis(5).await;
        delay(BLOCK_TIME);
        Timer::after_millis(5).await;
    }
}

// 用于模拟多任务执行环境，并且增加对比度
#[embassy_executor::task]
async fn task13() {
    loop {
        delay(BLOCK_TIME);
        Timer::after_millis(5).await;
        delay(BLOCK_TIME);
        Timer::after_millis(5).await;
    }
}

// 用于模拟多任务执行环境，并且增加对比度
#[embassy_executor::task]
async fn task14() {
    loop {
        delay(BLOCK_TIME);
        Timer::after_millis(5).await;
        delay(BLOCK_TIME);
        Timer::after_millis(5).await;
    }
}

// 用于模拟多任务执行环境，并且增加对比度
#[embassy_executor::task]
async fn task15() {
    loop {
        delay(BLOCK_TIME);
        Timer::after_millis(5).await;
        delay(BLOCK_TIME);
        Timer::after_millis(5).await;
    }
}

// 用于模拟多任务执行环境，并且增加对比度
#[embassy_executor::task]
async fn task16() {
    loop {
        delay(BLOCK_TIME);
        Timer::after_millis(5).await;
        delay(BLOCK_TIME);
        Timer::after_millis(5).await;
    }
}

// 用于模拟多任务执行环境，并且增加对比度
#[embassy_executor::task]
async fn task17() {
    loop {
        delay(BLOCK_TIME);
        Timer::after_millis(5).await;
        delay(BLOCK_TIME);
        Timer::after_millis(5).await;
    }
}

// 用于模拟多任务执行环境，并且增加对比度
#[embassy_executor::task]
async fn task18() {
    loop {
        delay(BLOCK_TIME);
        Timer::after_millis(5).await;
        delay(BLOCK_TIME);
        Timer::after_millis(5).await;
    }
}

// 用于模拟多任务执行环境，并且增加对比度
#[embassy_executor::task]
async fn task19() {
    loop {
        delay(BLOCK_TIME);
        Timer::after_millis(5).await;
        delay(BLOCK_TIME);
        Timer::after_millis(5).await;
    }
}

// 用于模拟多任务执行环境，并且增加对比度
#[embassy_executor::task]
async fn task20() {
    loop {
        delay(BLOCK_TIME);
        Timer::after_millis(5).await;
        delay(BLOCK_TIME);
        Timer::after_millis(5).await;
    }
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
