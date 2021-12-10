#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt_rtt as _; // global logger
use panic_probe as _;

use defmt::*;

use embassy::executor::Spawner;
use embassy::time::{Duration, Timer};
use embassy_nrf::gpio::{Level, Output, OutputDrive};
use embassy_nrf::Peripherals;
use embedded_hal::digital::v2::OutputPin;

#[embassy::task]
async fn blinker(mut led: Output<'static, P0_13>, interval: Duration) {
    loop {
        unwrap!(led.set_high());
        Timer::after(interval).await;
        unwrap!(led.set_low());
        Timer::after(interval).await;
    }
}

#[embassy::main]
async fn main(spawner: Spawner, p: Peripherals) {
    let led = Output::new(p.P0_13, Level::Low, OutputDrive::Standard);
    unwrap!(spawner.spawn(blinker(led, Duration::from_millis(300))));
}
