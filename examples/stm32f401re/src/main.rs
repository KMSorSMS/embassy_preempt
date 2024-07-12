//! This example showcases how to create multiple Executor instances to run tasks at
//! different priority levels.
//!
//! Low priority executor runs in thread mode (not interrupt), and uses `sev` for signaling
//! there's work in the queue, and `wfe` for waiting for work.
//!
//! Medium and high priority executors run in two interrupts with different priorities.
//! Signaling work is done by pending the interrupt. No "waiting" needs to be done explicitly, since
//! when there's work the interrupt will trigger and run the executor.
//!
//! Sample output below. Note that high priority ticks can interrupt everything else, and
//! medium priority computations can interrupt low priority computations, making them to appear
//! to take significantly longer time.
//!
//! ```not_rust
//!     [med] Starting long computation
//!     [med] done in 992 ms
//!         [high] tick!
//! [low] Starting long computation
//!     [med] Starting long computation
//!         [high] tick!
//!         [high] tick!
//!     [med] done in 993 ms
//!     [med] Starting long computation
//!         [high] tick!
//!         [high] tick!
//!     [med] done in 993 ms
//! [low] done in 3972 ms
//!     [med] Starting long computation
//!         [high] tick!
//!         [high] tick!
//!     [med] done in 993 ms
//! ```
//!
//! For comparison, try changing the code so all 3 tasks get spawned on the low priority executor.
//! You will get an output like the following. Note that no computation is ever interrupted.
//!
//! ```not_rust
//!         [high] tick!
//!     [med] Starting long computation
//!     [med] done in 496 ms
//! [low] Starting long computation
//! [low] done in 992 ms
//!     [med] Starting long computation
//!     [med] done in 496 ms
//!         [high] tick!
//! [low] Starting long computation
//! [low] done in 992 ms
//!         [high] tick!
//!     [med] Starting long computation
//!     [med] done in 496 ms
//!         [high] tick!
//! ```
//!

#![no_std]
#![no_main]

use embassy_stm32::rcc::Pll;
use stm32_metapac::rcc::vals;
use cortex_m_rt::entry;
use defmt::*;
use embassy_executor::{Executor, InterruptExecutor};
use embassy_stm32::interrupt;
use embassy_stm32::interrupt::{InterruptExt, Priority};
use embassy_time::{Instant, Timer};
use static_cell::StaticCell;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::task]
async fn run_high() {
    loop {
        info!("        [high] tick!");
        // Timer::after_millis(3000).await;
        let start = Instant::now();
        info!("    [high] Starting long computation");

        // Spin-wait to simulate a long CPU computation
        cortex_m::asm::delay(8_000_000*8); // ~1 second

        let end = Instant::now();
        let ms = end.duration_since(start).as_millis();
        info!("    [high] done in {} ms", ms);
        Timer::after_millis(3000).await;

    }
}

#[embassy_executor::task]
async fn run_med() {
    loop {
        let start = Instant::now();
        info!("    [med] Starting long computation");

        // Spin-wait to simulate a long CPU computation
        cortex_m::asm::delay(8_000_000*8); // ~1 second

        let end = Instant::now();
        let ms = end.duration_since(start).as_millis();
        info!("    [med] done in {} ms", ms);

        // Timer::after_millis(3000).await;
    }
}

#[embassy_executor::task]
async fn run_low() {
    loop {
        let start = Instant::now();
        info!("[low] Starting long computation");

        // Spin-wait to simulate a long CPU computation
        cortex_m::asm::delay(16_000_000*8); // ~2 seconds

        let end = Instant::now();
        let ms = end.duration_since(start).as_millis();
        info!("[low] done in {} ms", ms);

        Timer::after_millis(3000).await;
    }
}

static EXECUTOR_HIGH: InterruptExecutor = InterruptExecutor::new();
static EXECUTOR_MED: InterruptExecutor = InterruptExecutor::new();
static EXECUTOR_LOW: StaticCell<Executor> = StaticCell::new();

#[interrupt]
unsafe fn USART1() {
    EXECUTOR_HIGH.on_interrupt()
}

#[interrupt]
unsafe fn USART2() {
    EXECUTOR_MED.on_interrupt()
}

#[entry]
fn main() -> ! {
   // let p = embassy_stm32::init(Default::default());
    // we use 84Mhz sys from 8Mhz HSE with
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

    let _p = embassy_stm32::init(config);
    info!("Hello World!");

    // STM32s don’t have any interrupts exclusively for software use, but they can all be triggered by software as well as
    // by the peripheral, so we can just use any free interrupt vectors which aren’t used by the rest of your application.
    // In this case we’re using UART4 and UART5, but there’s nothing special about them. Any otherwise unused interrupt
    // vector would work exactly the same.

    // High-priority executor: USART1, priority level 6
    interrupt::USART1.set_priority(Priority::P6);
    let spawner = EXECUTOR_HIGH.start(interrupt::USART1);
    unwrap!(spawner.spawn(run_high()));

    // Medium-priority executor: UART5, priority level 7
    interrupt::USART2.set_priority(Priority::P7);
    let spawner = EXECUTOR_MED.start(interrupt::USART2);
    unwrap!(spawner.spawn(run_med()));

    // Low priority executor: runs in thread mode, using WFE/SEV
    let executor = EXECUTOR_LOW.init(Executor::new());
    executor.run(|spawner| {
        unwrap!(spawner.spawn(run_low()));
    });
}

// #![no_std]
// #![no_main]

// use defmt::*;
// use embassy_executor::Spawner;
// use embassy_stm32::{
//     gpio::{Level, Output, Speed},
//     rcc::Pll,
// };
// use embassy_time::Timer;
// use stm32_metapac::rcc::vals;
// use {defmt_rtt as _, panic_probe as _};
// #[embassy_executor::main]
// async fn main(_spawner: Spawner) {
//     // let p = embassy_stm32::init(Default::default());
//     // we use 84Mhz sys from 8Mhz HSE with
//     let hse = Some(embassy_stm32::rcc::Hse {
//         freq: embassy_stm32::time::Hertz(8_000_000),
//         mode: embassy_stm32::rcc::HseMode::Oscillator,
//     });
//     let pll = Some(Pll {
//         prediv: vals::Pllm::DIV4,
//         mul: vals::Plln::MUL84,
//         divp: Some(vals::Pllp::DIV2),
//         divq: Some(vals::Pllq::DIV4),
//         divr: None,
//     });

//     let mut rcc = embassy_stm32::rcc::Config::default();
//     // config the default mannually, its dull
//     rcc.hsi = false;
//     rcc.hse = hse;
//     rcc.sys = vals::Sw::PLL1_P;
//     rcc.pll_src = vals::Pllsrc::HSE;
//     rcc.pll = pll;
//     rcc.ahb_pre = vals::Hpre::DIV1;
//     rcc.apb1_pre = vals::Ppre::DIV2;
//     rcc.apb2_pre = vals::Ppre::DIV2;

//     let mut config = embassy_stm32::Config::default();
//     config.rcc = rcc;

//     let p = embassy_stm32::init(config);
//     info!("Hello World!");

//     let mut led = Output::new(p.PA5, Level::High, Speed::Low);

//     loop {
//         info!("high");
//         led.set_high();
//         Timer::after_millis(1000).await;

//         info!("low");
//         led.set_low();
//         Timer::after_millis(1000).await;
//     }
// }
