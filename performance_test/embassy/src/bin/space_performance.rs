#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_stm32::{gpio::{Level, Output, Speed}, rcc::Pll};
use embassy_time::Timer;
use stm32_metapac::{rcc,gpio};
use {defmt_rtt as _, panic_probe as _};
use stm32_metapac::{self, GPIOA, RCC};


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

    // let mut led = Output::new(p.PA5, Level::High, Speed::High);
    LED_Init();
    Pin_Init();
    // 创建任务
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

    // 主要测试任务,在空间利用率测试中，与其他任务无异
    loop {
        // led.set_high();
        Timer::after_millis(300).await;
        // button.wait_for_rising_edge().await;
        // led.set_low();
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

// 用于模拟多任务执行环境，并且增加对比度
#[embassy_executor::task]
async fn task6() {
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
async fn task7() {
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
async fn task8() {
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
async fn task9() {
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
async fn task10() {
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
async fn task11() {
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
async fn task12() {
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
async fn task13() {
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
async fn task14() {
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
async fn task15() {
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
async fn task16() {
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
async fn task17() {
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
async fn task18() {
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
async fn task19() {
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
async fn task20() {
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

/// init the LED
#[allow(dead_code)]
pub fn LED_Init() {
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
        v.set_ot(5,gpio::vals::Ot::PUSHPULL);
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
pub fn LED_ON() {
    GPIOA.odr().modify(|v| {
        v.set_odr(5, gpio::vals::Odr::HIGH);
    });
}

/// turn off the LED
#[allow(dead_code)]
#[inline]
pub fn LED_OFF() {
    GPIOA.odr().modify(|v| {
        v.set_odr(5, gpio::vals::Odr::LOW);
    });
}

/// TEST: thread pin and interrupt pin are used in the time_performance test
/// use the PA0 as the thread pin
/// use the PA1 as the interrupt pin
#[allow(dead_code)]
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

/// set the interrupt pin high
#[allow(dead_code)]
#[inline]
pub fn interrupt_pin_high() {
    GPIOA.odr().modify(|v| {
        v.set_odr(1, gpio::vals::Odr::HIGH);
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