#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_stm32::{gpio::{Level, Output, Speed}, rcc::Pll};
use embassy_time::Timer;
use stm32_metapac::rcc::vals;

// 主要测试任务
#[embassy_executor::main]
async fn main(spawner: Spawner) {
    // 硬件初始化
    let hse = Some(embassy_stm32::rcc::Hse {
        freq: embassy_stm32::time::Hertz(8_000_000),
        mode: embassy_stm32::rcc::HseMode::Oscillator,
    });
    let pll = Some(Pll {
        prediv: vals::Pllm::DIV4,
        mul: vals::Plln::MUL84,
        divp: Some(vals::Pllp::DIV2),
        divq: Some(vals::Pllq::DIV4),
        divr: None,
    });
    let mut rcc = embassy_stm32::rcc::Config::default();
    // config the default mannually, its dull
    rcc.hsi = false;
    rcc.hse = hse;
    rcc.sys = vals::Sw::PLL1_P;
    rcc.pll_src = vals::Pllsrc::HSE;
    rcc.pll = pll;
    rcc.ahb_pre = vals::Hpre::DIV1;
    rcc.apb1_pre = vals::Ppre::DIV2;
    rcc.apb2_pre = vals::Ppre::DIV2;

    let mut config = embassy_stm32::Config::default();
    config.rcc = rcc;
    let p = embassy_stm32::init(config);

    // info!("Hello World");

    let mut led = Output::new(p.PA5, Level::High, Speed::Low);

    // 创建任务
    spawner.spawn(task1());
    spawner.spawn(task2());
    spawner.spawn(task3());
    spawner.spawn(task4());
    spawner.spawn(task5());

    // 主要测试任务
    loop {
        led.set_high();
        Timer::after_millis(300).await;
        // button.wait_for_rising_edge().await;
        led.set_low();
        Timer::after_millis(300).await;
    }
}

// 用于模拟多任务执行环境，并且增加对比度
#[embassy_executor::task]
async fn task1() {
    loop {
        // led on
        // LED_ON();
        // delay(1);
        // delay 5s
        Timer::after_secs(5).await;
        // led off
        // LED_OFF();
        // delay(1);
        // delay 5s
        Timer::after_secs(5).await;
    }
}

// 用于模拟多任务执行环境，并且增加对比度
#[embassy_executor::task]
async fn task2() {
    loop {
        // led on
        // LED_ON();
        // delay(1);
        // delay 5s
        Timer::after_secs(5).await;
        // led off
        // LED_OFF();
        // delay(1);
        // delay 5s
        Timer::after_secs(5).await;
    }
}

// 用于模拟多任务执行环境，并且增加对比度
#[embassy_executor::task]
async fn task3() {
    loop {
        // led on
        // LED_ON();
        // delay(1);
        // delay 5s
        Timer::after_secs(5).await;
        // led off
        // LED_OFF();
        // delay(1);
        // delay 5s
        Timer::after_secs(5).await;
    }
}

// 用于模拟多任务执行环境，并且增加对比度
#[embassy_executor::task]
async fn task4() {
    loop {
        // led on
        // LED_ON();
        // delay(1);
        // delay 5s
        Timer::after_secs(5).await;
        // led off
        // LED_OFF();
        // delay(1);
        // delay 5s
        Timer::after_secs(5).await;
    }
}

// 用于模拟多任务执行环境，并且增加对比度
#[embassy_executor::task]
async fn task5() {
    loop {
        // led on
        // LED_ON();
        // delay(1);
        // delay 5s
        Timer::after_secs(5).await;
        // led off
        // LED_OFF();
        // delay(1);
        // delay 5s
        Timer::after_secs(5).await;
    }
}