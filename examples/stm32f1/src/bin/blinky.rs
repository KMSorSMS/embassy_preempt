#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::*;
use embassy_executor::executor::Spawner;
use embassy_executor::time::{Duration, Timer};
use embassy_stm32::gpio::{Level, Output, Speed};
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    info!("Hello World!");

    let mut led = Output::new(p.PC13, Level::High, Speed::Low);

    loop {
        info!("high");
        led.set_high();
        Timer::after(Duration::from_millis(300)).await;

        info!("low");
        led.set_low();
        Timer::after(Duration::from_millis(300)).await;
    }
}
