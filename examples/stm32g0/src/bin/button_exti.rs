#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::*;
use embassy_executor::executor::Spawner;
use embassy_stm32::exti::ExtiInput;
use embassy_stm32::gpio::{Input, Pull};
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    info!("Hello World!");

    let button = Input::new(p.PC13, Pull::Up);
    let mut button = ExtiInput::new(button, p.EXTI13);

    info!("Press the USER button...");

    loop {
        button.wait_for_falling_edge().await;
        info!("Pressed!");
        button.wait_for_rising_edge().await;
        info!("Released!");
    }
}
