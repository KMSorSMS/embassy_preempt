#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::*;
use embassy_executor::Spawner;
use embassy_nrf::{interrupt, uarte};
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_nrf::init(Default::default());
    let mut config = uarte::Config::default();
    config.parity = uarte::Parity::EXCLUDED;
    config.baudrate = uarte::Baudrate::BAUD115200;

    let irq = interrupt::take!(SERIAL0);
    let uart = uarte::Uarte::new(p.UARTETWISPI0, irq, p.P0_08, p.P0_06, config);
    let (mut tx, mut rx) = uart.split_with_idle(p.TIMER0, p.PPI_CH0, p.PPI_CH1);

    info!("uarte initialized!");

    // Message must be in SRAM
    let mut buf = [0; 8];
    buf.copy_from_slice(b"Hello!\r\n");

    unwrap!(tx.write(&buf).await);
    info!("wrote hello in uart!");

    loop {
        info!("reading...");
        let n = unwrap!(rx.read_until_idle(&mut buf).await);
        info!("got {} bytes", n);
    }
}
