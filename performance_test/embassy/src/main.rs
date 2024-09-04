#![no_std]
#![no_main]

use {defmt_rtt as _, panic_probe as _, stm32_metapac as _};

#[cortex_m_rt::entry]
fn main() -> ! {
    loop {}
}
