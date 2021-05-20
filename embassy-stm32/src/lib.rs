#![no_std]
#![feature(generic_associated_types)]
#![feature(asm)]
#![feature(min_type_alias_impl_trait)]
#![feature(impl_trait_in_bindings)]
#![feature(type_alias_impl_trait)]
#![allow(incomplete_features)]

// This must go FIRST so that all the other modules see its macros.
pub mod fmt;

#[cfg(feature = "_dma")]
pub mod dma;
pub mod exti;
pub mod gpio;
#[cfg(feature = "_rng")]
pub mod rng;
#[cfg(feature = "_sdmmc")]
pub mod sdmmc;
#[cfg(feature = "_spi")]
pub mod spi;
#[cfg(feature = "_usart")]
pub mod usart;

// This must go LAST so that it sees the `impl_foo!` macros
mod pac;

pub mod time;

pub use embassy_macros::interrupt;
pub use pac::{interrupt, peripherals, Peripherals};

// workaround for svd2rust-generated code using `use crate::generic::*;`
pub(crate) use pac::generic;

#[non_exhaustive]
pub struct Config {
    _private: (),
}

impl Default for Config {
    fn default() -> Self {
        Self { _private: () }
    }
}

/// Initialize embassy.
pub fn init(_config: Config) -> Peripherals {
    let p = Peripherals::take();

    unsafe {
        dma::init();
        pac::init_exti();
    }

    p
}
