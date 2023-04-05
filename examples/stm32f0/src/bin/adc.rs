#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::adc::{Adc, SampleTime};
use embassy_time::{Delay, Duration, Timer};
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    info!("Hello World!");

    let mut adc = Adc::new(p.ADC, &mut Delay);
    adc.set_sample_time(SampleTime::Cycles71_5);
    let mut pin = p.PA1;

    let mut vrefint = adc.enable_vref(&mut Delay);
    let vrefint_sample = adc.read_internal(&mut vrefint);
    let convert_to_millivolts = |sample| {
        // FIXME: use proper datasheet and value
        // From http://www.st.com/resource/en/datasheet/CD00161566.pdf
        // 5.3.4 Embedded reference voltage
        const VREFINT_MV: u32 = 1200; // mV

        (u32::from(sample) * VREFINT_MV / u32::from(vrefint_sample)) as u16
    };

    loop {
        let v = adc.read(&mut pin);
        info!("--> {} - {} mV", v, convert_to_millivolts(v));
        Timer::after(Duration::from_millis(100)).await;
    }
}
