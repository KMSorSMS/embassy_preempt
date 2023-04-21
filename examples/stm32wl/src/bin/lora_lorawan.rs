//! This example runs on a STM32WL board, which has a builtin Semtech Sx1262 radio.
//! It demonstrates LoRaWAN functionality.
#![no_std]
#![no_main]
#![macro_use]
#![feature(type_alias_impl_trait, async_fn_in_trait)]
#![allow(incomplete_features)]

use defmt::info;
use embassy_embedded_hal::adapter::BlockingAsync;
use embassy_executor::Spawner;
use embassy_lora::iv::Stm32wlInterfaceVariant;
use embassy_lora::LoraTimer;
use embassy_stm32::dma::NoDma;
use embassy_stm32::gpio::{Level, Output, Pin, Speed};
use embassy_stm32::peripherals::SUBGHZSPI;
use embassy_stm32::rcc::low_level::RccPeripheral;
use embassy_stm32::rng::Rng;
use embassy_stm32::spi::{BitOrder, Config as SpiConfig, Spi, MODE_0};
use embassy_stm32::time::Hertz;
use embassy_stm32::{interrupt, into_ref, pac, Peripheral};
use embassy_time::Delay;
use lora_phy::mod_params::*;
use lora_phy::sx1261_2::SX1261_2;
use lora_phy::LoRa;
use lorawan::default_crypto::DefaultFactory as Crypto;
use lorawan_device::async_device::lora_radio::LoRaRadio;
use lorawan_device::async_device::{region, Device, JoinMode};
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let mut config = embassy_stm32::Config::default();
    config.rcc.mux = embassy_stm32::rcc::ClockSrc::HSI16;
    config.rcc.enable_lsi = true;
    let p = embassy_stm32::init(config);

    unsafe { pac::RCC.ccipr().modify(|w| w.set_rngsel(0b01)) }

    let clk = Hertz(core::cmp::min(SUBGHZSPI::frequency().0 / 2, 16_000_000));
    let mut spi_config = SpiConfig::default();
    spi_config.mode = MODE_0;
    spi_config.bit_order = BitOrder::MsbFirst;
    let spi = Spi::new_subghz(p.SUBGHZSPI, NoDma, NoDma, clk, spi_config);

    let spi = BlockingAsync::new(spi);

    let irq = interrupt::take!(SUBGHZ_RADIO);
    into_ref!(irq);
    // Set CTRL1 and CTRL3 for high-power transmission, while CTRL2 acts as an RF switch between tx and rx
    let _ctrl1 = Output::new(p.PC4.degrade(), Level::Low, Speed::High);
    let ctrl2 = Output::new(p.PC5.degrade(), Level::High, Speed::High);
    let _ctrl3 = Output::new(p.PC3.degrade(), Level::High, Speed::High);
    let iv = Stm32wlInterfaceVariant::new(irq, None, Some(ctrl2)).unwrap();

    let mut delay = Delay;

    let lora = {
        match LoRa::new(SX1261_2::new(BoardType::Stm32wlSx1262, spi, iv), true, &mut delay).await {
            Ok(l) => l,
            Err(err) => {
                info!("Radio error = {}", err);
                return;
            }
        }
    };
    let radio = LoRaRadio::new(lora);
    let region: region::Configuration = region::Configuration::new(region::Region::EU868);
    let mut device: Device<_, Crypto, _, _> = Device::new(region, radio, LoraTimer::new(), Rng::new(p.RNG));

    defmt::info!("Joining LoRaWAN network");

    // TODO: Adjust the EUI and Keys according to your network credentials
    match device
        .join(&JoinMode::OTAA {
            deveui: [0, 0, 0, 0, 0, 0, 0, 0],
            appeui: [0, 0, 0, 0, 0, 0, 0, 0],
            appkey: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        })
        .await
    {
        Ok(()) => defmt::info!("LoRaWAN network joined"),
        Err(err) => {
            info!("Radio error = {}", err);
            return;
        }
    };
}
