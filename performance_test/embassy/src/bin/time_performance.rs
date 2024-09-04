#![no_std]
#![no_main]

use core::arch::asm;

use embassy_executor::Spawner;
use embassy_stm32::{exti::ExtiInput, gpio::{Level, Output, Pull, Speed}, rcc::Pll};
use embassy_time::Timer;
use stm32_metapac::{gpio, rcc::vals, GPIOA, RCC};

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

    let led: Output<'static> = Output::new(p.PA5, Level::High, Speed::High);

    // 初始化按键，以及对应中断。
    let mut button = ExtiInput::new(p.PC13, p.EXTI13,Pull::Down);

    // 初始化process pin 与interrupt pin
    Pin_Init();

    // 创建任务
    spawner.spawn(task1(led));
    spawner.spawn(task2());
    spawner.spawn(task3());
    spawner.spawn(task4());
    spawner.spawn(task5());

    // 主要测试任务
    loop {
        button.wait_for_rising_edge().await;
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

// 用于模拟多任务执行环境，并且增加对比度
#[embassy_executor::task]
async fn task1(mut led: Output<'static>) {
    loop {
        // 将闪灯代码放入task1以免影响引脚设置和对Timer delay的测量
        led.set_high();
        Timer::after_secs(5).await;
        led.set_low();
        Timer::after_secs(5).await;
    }
}

// 用于模拟多任务执行环境，并且增加对比度
#[embassy_executor::task]
async fn task2() {
    loop {
        delay(6);
        Timer::after_secs(5).await;
        delay(6);
        Timer::after_secs(5).await;
    }
}

// 用于模拟多任务执行环境，并且增加对比度
#[embassy_executor::task]
async fn task3() {
    loop {
        delay(6);
        Timer::after_secs(5).await;
        delay(6);
        Timer::after_secs(5).await;
    }
}

// 用于模拟多任务执行环境，并且增加对比度
#[embassy_executor::task]
async fn task4() {
    loop {
        delay(6);
        Timer::after_secs(5).await;
        delay(6);
        Timer::after_secs(5).await;
    }
}

// 用于模拟多任务执行环境，并且增加对比度
#[embassy_executor::task]
async fn task5() {
    loop {
        delay(6);
        Timer::after_secs(5).await;
        delay(6);
        Timer::after_secs(5).await;
    }
}

/// TEST: thread pin and interrupt pin are used in the time_performance test
/// use the PA0 as the thread pin
/// use the PA1 as the interrupt pin
#[allow(non_snake_case)]
pub fn Pin_Init(){
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
#[inline]
pub fn thread_pin_high() {
    GPIOA.odr().modify(|v| {
        v.set_odr(0, gpio::vals::Odr::HIGH);
    });
}

/// set the thread pin low
#[inline]
pub fn thread_pin_low() {
    GPIOA.odr().modify(|v| {
        v.set_odr(0, gpio::vals::Odr::LOW);
    });
}

/// set the interrupt pin high
#[inline]
pub fn interrupt_pin_high() {
    GPIOA.odr().modify(|v| {
        v.set_odr(1, gpio::vals::Odr::HIGH);
    });
}

/// set the interrupt pin low
#[inline]
pub fn interrupt_pin_low() {
    GPIOA.odr().modify(|v| {
        v.set_odr(1, gpio::vals::Odr::LOW);
    });
}

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