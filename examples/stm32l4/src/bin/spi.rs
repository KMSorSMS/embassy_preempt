#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::*;
use embassy_stm32::dma::NoDma;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_stm32::spi::{Config, Spi};
use {defmt_rtt as _, panic_probe as _};

#[cortex_m_rt::entry]
fn main() -> ! {
    info!("Hello World!");

    let p = embassy_stm32::init(Default::default());

    let mut spi = Spi::new(p.SPI3, p.PC10, p.PC12, p.PC11, NoDma, NoDma, Config::default());

    let mut cs = Output::new(p.PE0, Level::High, Speed::VeryHigh);

    loop {
        let mut buf = [0x0Au8; 4];
        cs.set_low();
        unwrap!(spi.blocking_transfer_in_place(&mut buf));
        cs.set_high();
        info!("xfer {=[u8]:x}", buf);
    }
}
