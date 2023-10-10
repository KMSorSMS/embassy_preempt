pub use crate::pac::rcc::vals::{
    Hpre as AHBPrescaler, Msirange as MSIRange, Plldiv as PLLDiv, Pllmul as PLLMul, Ppre as APBPrescaler,
};
use crate::pac::rcc::vals::{Pllsrc, Sw};
use crate::pac::{FLASH, RCC};
use crate::rcc::{set_freqs, Clocks};
use crate::time::Hertz;

/// HSI speed
pub const HSI_FREQ: Hertz = Hertz(16_000_000);

/// LSI speed
pub const LSI_FREQ: Hertz = Hertz(32_000);

/// System clock mux source
#[derive(Clone, Copy)]
pub enum ClockSrc {
    MSI(MSIRange),
    PLL(PLLSource, PLLMul, PLLDiv),
    HSE(Hertz),
    HSI,
}

/// PLL clock input source
#[derive(Clone, Copy)]
pub enum PLLSource {
    HSI,
    HSE(Hertz),
}

impl From<PLLSource> for Pllsrc {
    fn from(val: PLLSource) -> Pllsrc {
        match val {
            PLLSource::HSI => Pllsrc::HSI,
            PLLSource::HSE(_) => Pllsrc::HSE,
        }
    }
}

/// Clocks configutation
pub struct Config {
    pub mux: ClockSrc,
    pub ahb_pre: AHBPrescaler,
    pub apb1_pre: APBPrescaler,
    pub apb2_pre: APBPrescaler,
}

impl Default for Config {
    #[inline]
    fn default() -> Config {
        Config {
            mux: ClockSrc::MSI(MSIRange::RANGE5),
            ahb_pre: AHBPrescaler::DIV1,
            apb1_pre: APBPrescaler::DIV1,
            apb2_pre: APBPrescaler::DIV1,
        }
    }
}

pub(crate) unsafe fn init(config: Config) {
    let (sys_clk, sw) = match config.mux {
        ClockSrc::MSI(range) => {
            // Set MSI range
            RCC.icscr().write(|w| w.set_msirange(range));

            // Enable MSI
            RCC.cr().write(|w| w.set_msion(true));
            while !RCC.cr().read().msirdy() {}

            let freq = 32_768 * (1 << (range as u8 + 1));
            (Hertz(freq), Sw::MSI)
        }
        ClockSrc::HSI => {
            // Enable HSI
            RCC.cr().write(|w| w.set_hsion(true));
            while !RCC.cr().read().hsirdy() {}

            (HSI_FREQ, Sw::HSI)
        }
        ClockSrc::HSE(freq) => {
            // Enable HSE
            RCC.cr().write(|w| w.set_hseon(true));
            while !RCC.cr().read().hserdy() {}

            (freq, Sw::HSE)
        }
        ClockSrc::PLL(src, mul, div) => {
            let freq = match src {
                PLLSource::HSE(freq) => {
                    // Enable HSE
                    RCC.cr().write(|w| w.set_hseon(true));
                    while !RCC.cr().read().hserdy() {}
                    freq
                }
                PLLSource::HSI => {
                    // Enable HSI
                    RCC.cr().write(|w| w.set_hsion(true));
                    while !RCC.cr().read().hsirdy() {}
                    HSI_FREQ
                }
            };

            // Disable PLL
            RCC.cr().modify(|w| w.set_pllon(false));
            while RCC.cr().read().pllrdy() {}

            let freq = freq * mul / div;

            assert!(freq <= Hertz(32_000_000));

            RCC.cfgr().write(move |w| {
                w.set_pllmul(mul);
                w.set_plldiv(div);
                w.set_pllsrc(src.into());
            });

            // Enable PLL
            RCC.cr().modify(|w| w.set_pllon(true));
            while !RCC.cr().read().pllrdy() {}

            (freq, Sw::PLL)
        }
    };

    // Set flash 64-bit access, prefetch and wait states
    if sys_clk >= Hertz(16_000_000) {
        FLASH.acr().write(|w| w.set_acc64(true));
        FLASH.acr().modify(|w| w.set_prften(true));
        FLASH.acr().modify(|w| w.set_latency(true));
    }

    RCC.cfgr().modify(|w| {
        w.set_sw(sw);
        w.set_hpre(config.ahb_pre);
        w.set_ppre1(config.apb1_pre);
        w.set_ppre2(config.apb2_pre);
    });

    let ahb_freq = sys_clk / config.ahb_pre;

    let (apb1_freq, apb1_tim_freq) = match config.apb1_pre {
        APBPrescaler::DIV1 => (ahb_freq, ahb_freq),
        pre => {
            let freq = ahb_freq / pre;
            (freq, freq * 2u32)
        }
    };

    let (apb2_freq, apb2_tim_freq) = match config.apb2_pre {
        APBPrescaler::DIV1 => (ahb_freq, ahb_freq),
        pre => {
            let freq = ahb_freq / pre;
            (freq, freq * 2u32)
        }
    };

    #[cfg(crs)]
    if config.enable_hsi48 {
        // Reset CRS peripheral
        RCC.apb1rstr().modify(|w| w.set_crsrst(true));
        RCC.apb1rstr().modify(|w| w.set_crsrst(false));

        // Enable CRS peripheral
        RCC.apb1enr().modify(|w| w.set_crsen(true));

        // Initialize CRS
        CRS.cfgr().write(|w|

        // Select LSE as synchronization source
        w.set_syncsrc(crs::vals::Syncsrc::LSE));
        CRS.cr().modify(|w| {
            w.set_autotrimen(true);
            w.set_cen(true);
        });

        // Enable VREFINT reference for HSI48 oscillator
        SYSCFG.cfgr3().modify(|w| {
            w.set_enref_hsi48(true);
            w.set_en_vrefint(true);
        });

        // Select HSI48 as USB clock
        RCC.ccipr().modify(|w| w.set_hsi48msel(true));

        // Enable dedicated USB clock
        RCC.crrcr().modify(|w| w.set_hsi48on(true));
        while !RCC.crrcr().read().hsi48rdy() {}
    }

    set_freqs(Clocks {
        sys: sys_clk,
        ahb1: ahb_freq,
        apb1: apb1_freq,
        apb2: apb2_freq,
        apb1_tim: apb1_tim_freq,
        apb2_tim: apb2_tim_freq,
    });
}
