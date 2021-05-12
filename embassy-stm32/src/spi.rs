#![macro_use]

pub use embedded_hal::spi::{Mode, Phase, Polarity, MODE_0, MODE_1, MODE_2, MODE_3};
use core::marker::PhantomData;
use embassy::interrupt::Interrupt;
use embedded_hal::blocking::spi::{Write, Transfer};
use embassy::util::Unborrow;
use embassy_extras::{impl_unborrow, unborrow};
use crate::gpio::{Pin, AnyPin};
use crate::pac::gpio::vals::{Afr, Moder};
use crate::pac::spi;
use crate::pac::gpio::Gpio;
use crate::time::Hertz;
//use crate::pac::spi;

// TODO move upwards in the tree
pub enum ByteOrder {
    LsbFirst,
    MsbFirst
}

pub struct Spi<'d, T: Instance> {
    peri: T,
    sck: AnyPin,
    mosi: AnyPin,
    miso: AnyPin,
    //irq: T::Interrupt,
    phantom: PhantomData<&'d mut T>,
}

impl<'d, T: Instance> Spi<'d, T> {
    pub fn new<F>(pclk: Hertz,
                  peri: impl Unborrow<Target=T> + 'd,
                  sck: impl Unborrow<Target=impl Sck<T>>,
                  mosi: impl Unborrow<Target=impl Mosi<T>>,
                  miso: impl Unborrow<Target=impl Miso<T>>,
                  mode: Mode,
                  byte_order: ByteOrder,
                  freq: F,
    ) -> Self
        where
            F: Into<Hertz>
    {
        unborrow!(peri);
        unborrow!(sck, mosi, miso);

        unsafe {
            Self::configure_pin(sck.block(), sck.pin() as _, sck.af());
            Self::configure_pin(mosi.block(), mosi.pin() as _, mosi.af());
            Self::configure_pin(miso.block(), miso.pin() as _, miso.af());
        }

        let sck = sck.degrade();
        let mosi = mosi.degrade();
        let miso = miso.degrade();

        // FRXTH: RXNE event is generated if the FIFO level is greater than or equal to
        //        8-bit
        // DS: 8-bit data size
        // SSOE: Slave Select output disabled
        unsafe {
            T::regs().cr2()
                .write(|w| {
                    // 8-bit transfers
                    w.set_ds( spi::vals::Ds(0b0111));
                    w.set_frxth( spi::vals::Frxth::QUARTER);
                    w.set_ssoe(false);
                });
        }

        let br = Self::compute_baud_rate(pclk, freq.into());

        unsafe {
            T::regs().cr1().write(|w| {
                w.set_cpha(
                    match mode.phase == Phase::CaptureOnSecondTransition {
                        true => spi::vals::Cpha::SECONDEDGE,
                        false => spi::vals::Cpha::FIRSTEDGE,
                    }
                );
                w.set_cpol(match mode.polarity == Polarity::IdleHigh {
                    true => spi::vals::Cpol::IDLEHIGH,
                    false => spi::vals::Cpol::IDLELOW,
                });

                w.set_mstr(spi::vals::Mstr::MASTER);
                w.set_br(spi::vals::Br(br));
                w.set_spe(true);
                w.set_lsbfirst(
                    match byte_order {
                        ByteOrder::LsbFirst => spi::vals::Lsbfirst::LSBFIRST,
                        ByteOrder::MsbFirst => spi::vals::Lsbfirst::MSBFIRST,
                    }
                );
                w.set_ssi(true);
                w.set_ssm(true);
                w.set_crcen(false);
                w.set_bidimode(spi::vals::Bidimode::UNIDIRECTIONAL);
            });
            T::regs().cr2().write(|w| {
            })
        }

        Self {
            peri,
            sck,
            mosi,
            miso,
            phantom: PhantomData,
        }
    }

    unsafe fn configure_pin(block: Gpio, pin: usize, af_num: u8) {
        let (afr, n_af) = if pin < 8 { (0, pin) } else { (1, pin - 8) };
        block.moder().modify(|w| w.set_moder(pin, Moder::ALTERNATE));
        block.afr(afr).modify(|w| w.set_afr(n_af, Afr(af_num)));
    }

    unsafe fn unconfigure_pin(block: Gpio, pin: usize) {
        let (afr, n_af) = if pin < 8 { (0, pin) } else { (1, pin - 8) };
        block.moder().modify(|w| w.set_moder(pin, Moder::ANALOG));
    }

    fn compute_baud_rate(clocks: Hertz, freq: Hertz) -> u8 {
        match clocks.0 / freq.0 {
            0 => unreachable!(),
            1..=2 => 0b000,
            3..=5 => 0b001,
            6..=11 => 0b010,
            12..=23 => 0b011,
            24..=39 => 0b100,
            40..=95 => 0b101,
            96..=191 => 0b110,
            _ => 0b111,
        }
    }
}

impl<'d, T: Instance> Drop for Spi<'d, T> {
    fn drop(&mut self) {
        unsafe {
            Self::unconfigure_pin(self.sck.block(), self.sck.pin() as _);
            Self::unconfigure_pin(self.mosi.block(), self.mosi.pin() as _);
            Self::unconfigure_pin(self.miso.block(), self.miso.pin() as _);
        }
    }
}

pub enum Error {
    Framing,
    Crc,
    Overrun,
}

impl<'d, T: Instance> embedded_hal::blocking::spi::Write<u8> for Spi<'d, T> {
    type Error = Error;

    fn write(&mut self, words: &[u8]) -> Result<(), Self::Error> {
        let regs = T::regs();

        for word in words.iter() {
            while unsafe { !regs.sr().read().txe() } {
                // spin
            }
            unsafe {
                regs.dr().write(|reg| reg.0 = *word as u32);
            }
            loop {
                let sr = unsafe { regs.sr().read() };
                if sr.fre() {
                    return Err(Error::Framing);
                }
                if sr.ovr() {
                    return Err(Error::Overrun);
                }
                if sr.crcerr() {
                    return Err(Error::Crc);
                }
                if !sr.txe() {
                    // loop waiting for TXE
                }
            }
        }

        Ok(())
    }
}

impl<'d, T: Instance> embedded_hal::blocking::spi::Transfer<u8> for Spi<'d, T> {
    type Error = Error;

    fn transfer<'w>(&mut self, words: &'w mut [u8]) -> Result<&'w [u8], Self::Error> {
        let regs = T::regs();

        for word in words.iter_mut() {
            while unsafe { !regs.sr().read().txe() } {
                // spin
            }
            unsafe {
                regs.dr().write(|reg| reg.0 = *word as u32);
            }
            while unsafe { !regs.sr().read().rxne() } {
                // spin waiting for inbound to shift in.
            }
            *word = unsafe { regs.dr().read().0 as u8 };
            loop {
                let sr = unsafe { regs.sr().read() };
                if sr.fre() {
                    return Err(Error::Framing);
                }
                if sr.ovr() {
                    return Err(Error::Overrun);
                }
                if sr.crcerr() {
                    return Err(Error::Crc);
                }
                if !sr.txe() {
                    // loop waiting for TXE
                }
            }
        }

        Ok(words)
    }
}


pub(crate) mod sealed {
    use super::*;
    use embassy::util::AtomicWaker;

    //pub struct State {
    //pub end_waker: AtomicWaker,
    //}

    //impl State {
    //pub const fn new() -> Self {
    //Self {
    //end_waker: AtomicWaker::new(),
    //}
    //}
    //}

    pub trait Instance {
        fn regs() -> &'static spi::Spi;
        //fn state() -> &'static State;
    }

    pub trait Sck<T: Instance>: Pin {
        const AF: u8;
        fn af(&self) -> u8 {
            Self::AF
        }
    }

    pub trait Mosi<T: Instance>: Pin {
        const AF: u8;
        fn af(&self) -> u8 {
            Self::AF
        }
    }

    pub trait Miso<T: Instance>: Pin {
        const AF: u8;
        fn af(&self) -> u8 {
            Self::AF
        }
    }
}

pub trait Instance: sealed::Instance + 'static {
    //type Interrupt: Interrupt;
}

pub trait Sck<T: Instance>: sealed::Sck<T> + 'static {}

pub trait Mosi<T: Instance>: sealed::Mosi<T> + 'static {}

pub trait Miso<T: Instance>: sealed::Miso<T> + 'static {}

macro_rules! impl_spi {
    ($inst:ident, $clk:ident) => {
        impl crate::spi::sealed::Instance for peripherals::$inst {
            fn regs() -> &'static crate::pac::spi::Spi {
                &crate::pac::$inst
            }
        }

        impl crate::spi::Instance for peripherals::$inst {}
    };
}

macro_rules! impl_spi_pin {
    ($inst:ident, $pin_func:ident, $pin:ident, $af:expr) => {
        impl crate::spi::$pin_func<peripherals::$inst> for peripherals::$pin {
        }

        impl crate::spi::sealed::$pin_func<peripherals::$inst> for peripherals::$pin {
            const AF: u8 = $af;
        }
    }
}