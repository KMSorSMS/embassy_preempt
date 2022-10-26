#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::flash::{ERASE_SIZE, FLASH_BASE};
use embassy_rp::peripherals::FLASH;
use embedded_storage::nor_flash::{NorFlash, ReadNorFlash};
use {defmt_rtt as _, panic_probe as _};

const ADDR_OFFSET: u32 = 0x100000;
const FLASH_SIZE: usize = 2 * 1024 * 1024;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    info!("Hello World!");

    let mut flash = embassy_rp::flash::Flash::<_, FLASH_SIZE>::new(p.FLASH);

    erase_write_sector(&mut flash, 0x00);

    multiwrite_bytes(&mut flash, ERASE_SIZE as u32);

    loop {}
}

fn multiwrite_bytes(flash: &mut embassy_rp::flash::Flash<'_, FLASH, FLASH_SIZE>, offset: u32) {
    info!(">>>> [multiwrite_bytes]");
    let mut read_buf = [0u8; ERASE_SIZE];
    defmt::unwrap!(flash.read(ADDR_OFFSET + offset, &mut read_buf));

    info!("Addr of flash block is {:x}", ADDR_OFFSET + offset + FLASH_BASE as u32);
    info!("Contents start with {=[u8]}", read_buf[0..4]);

    defmt::unwrap!(flash.erase(ADDR_OFFSET + offset, ADDR_OFFSET + offset + ERASE_SIZE as u32));

    defmt::unwrap!(flash.read(ADDR_OFFSET + offset, &mut read_buf));
    info!("Contents after erase starts with {=[u8]}", read_buf[0..4]);
    if read_buf.iter().any(|x| *x != 0xFF) {
        defmt::panic!("unexpected");
    }

    defmt::unwrap!(flash.write(ADDR_OFFSET + offset, &[0x01]));
    defmt::unwrap!(flash.write(ADDR_OFFSET + offset + 1, &[0x02]));
    defmt::unwrap!(flash.write(ADDR_OFFSET + offset + 2, &[0x03]));
    defmt::unwrap!(flash.write(ADDR_OFFSET + offset + 3, &[0x04]));

    defmt::unwrap!(flash.read(ADDR_OFFSET + offset, &mut read_buf));
    info!("Contents after write starts with {=[u8]}", read_buf[0..4]);
    if &read_buf[0..4] != &[0x01, 0x02, 0x03, 0x04] {
        defmt::panic!("unexpected");
    }
}

fn erase_write_sector(flash: &mut embassy_rp::flash::Flash<'_, FLASH, FLASH_SIZE>, offset: u32) {
    info!(">>>> [erase_write_sector]");
    let mut buf = [0u8; ERASE_SIZE];
    defmt::unwrap!(flash.read(ADDR_OFFSET + offset, &mut buf));

    info!("Addr of flash block is {:x}", ADDR_OFFSET + offset + FLASH_BASE as u32);
    info!("Contents start with {=[u8]}", buf[0..4]);

    defmt::unwrap!(flash.erase(ADDR_OFFSET + offset, ADDR_OFFSET + offset + ERASE_SIZE as u32));

    defmt::unwrap!(flash.read(ADDR_OFFSET + offset, &mut buf));
    info!("Contents after erase starts with {=[u8]}", buf[0..4]);
    if buf.iter().any(|x| *x != 0xFF) {
        defmt::panic!("unexpected");
    }

    for b in buf.iter_mut() {
        *b = 0xDA;
    }

    defmt::unwrap!(flash.write(ADDR_OFFSET + offset, &buf));

    defmt::unwrap!(flash.read(ADDR_OFFSET + offset, &mut buf));
    info!("Contents after write starts with {=[u8]}", buf[0..4]);
    if buf.iter().any(|x| *x != 0xDA) {
        defmt::panic!("unexpected");
    }
}
