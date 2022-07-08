//! Blocking shared SPI bus
//!
//! # Example (nrf52)
//!
//! ```rust
//! use embassy_embedded_hal::shared_bus::blocking::spi::SpiBusDevice;
//! use embassy::blocking_mutex::{NoopMutex, raw::NoopRawMutex};
//!
//! static SPI_BUS: Forever<NoopMutex<RefCell<Spim<SPI3>>>> = Forever::new();
//! let irq = interrupt::take!(SPIM3);
//! let spi = Spim::new_txonly(p.SPI3, irq, p.P0_15, p.P0_18, Config::default());
//! let spi_bus = NoopMutex::new(RefCell::new(spi));
//! let spi_bus = SPI_BUS.put(spi_bus);
//!
//! // Device 1, using embedded-hal compatible driver for ST7735 LCD display
//! let cs_pin1 = Output::new(p.P0_24, Level::Low, OutputDrive::Standard);
//! let spi_dev1 = SpiBusDevice::new(spi_bus, cs_pin1);
//! let display1 = ST7735::new(spi_dev1, dc1, rst1, Default::default(), false, 160, 128);
//! ```

use core::cell::RefCell;

use embassy::blocking_mutex::raw::RawMutex;
use embassy::blocking_mutex::Mutex;
use embedded_hal_1::digital::blocking::OutputPin;
use embedded_hal_1::spi;
use embedded_hal_1::spi::blocking::{SpiBusFlush, SpiDevice};

use crate::shared_bus::spi::SpiBusDeviceError;
use crate::SetConfig;

pub struct SpiBusDevice<'a, M: RawMutex, BUS, CS> {
    bus: &'a Mutex<M, RefCell<BUS>>,
    cs: CS,
}

impl<'a, M: RawMutex, BUS, CS> SpiBusDevice<'a, M, BUS, CS> {
    pub fn new(bus: &'a Mutex<M, RefCell<BUS>>, cs: CS) -> Self {
        Self { bus, cs }
    }
}

impl<'a, M: RawMutex, BUS, CS> spi::ErrorType for SpiBusDevice<'a, M, BUS, CS>
where
    BUS: spi::ErrorType,
    CS: OutputPin,
{
    type Error = SpiBusDeviceError<BUS::Error, CS::Error>;
}

impl<BUS, M, CS> SpiDevice for SpiBusDevice<'_, M, BUS, CS>
where
    M: RawMutex,
    BUS: SpiBusFlush,
    CS: OutputPin,
{
    type Bus = BUS;

    fn transaction<R>(&mut self, f: impl FnOnce(&mut Self::Bus) -> Result<R, BUS::Error>) -> Result<R, Self::Error> {
        self.bus.lock(|bus| {
            let mut bus = bus.borrow_mut();
            self.cs.set_low().map_err(SpiBusDeviceError::Cs)?;

            let f_res = f(&mut bus);

            // On failure, it's important to still flush and deassert CS.
            let flush_res = bus.flush();
            let cs_res = self.cs.set_high();

            let f_res = f_res.map_err(SpiBusDeviceError::Spi)?;
            flush_res.map_err(SpiBusDeviceError::Spi)?;
            cs_res.map_err(SpiBusDeviceError::Cs)?;

            Ok(f_res)
        })
    }
}

<<<<<<< HEAD
pub struct SpiBusDeviceWithConfig<'a, M: RawMutex, BUS, CS, C> {
    bus: &'a Mutex<M, RefCell<BUS>>,
    cs: CS,
    config: C,
}

impl<'a, M: RawMutex, BUS, CS, C> SpiBusDeviceWithConfig<'a, M, BUS, CS, C> {
    pub fn new(bus: &'a Mutex<M, RefCell<BUS>>, cs: CS, config: C) -> Self {
        Self { bus, cs, config }
    }
}

impl<'a, M: RawMutex, BUS, CS, C> spi::ErrorType for SpiBusDeviceWithConfig<'a, M, BUS, CS, C>
where
    BUS: spi::ErrorType,
    CS: OutputPin,
{
    type Error = SpiBusDeviceError<BUS::Error, CS::Error>;
}

impl<BUS, M, CS, C> SpiDevice for SpiBusDeviceWithConfig<'_, M, BUS, CS, C>
where
    M: RawMutex,
    BUS: SpiBusFlush + SetConfig<C>,
    CS: OutputPin,
{
    type Bus = BUS;

    fn transaction<R>(&mut self, f: impl FnOnce(&mut Self::Bus) -> Result<R, BUS::Error>) -> Result<R, Self::Error> {
        self.bus.lock(|bus| {
            let mut bus = bus.borrow_mut();
            bus.set_config(&self.config);
            self.cs.set_low().map_err(SpiBusDeviceError::Cs)?;

            let f_res = f(&mut bus);

            // On failure, it's important to still flush and deassert CS.
            let flush_res = bus.flush();
            let cs_res = self.cs.set_high();

            let f_res = f_res.map_err(SpiBusDeviceError::Spi)?;
            flush_res.map_err(SpiBusDeviceError::Spi)?;
            cs_res.map_err(SpiBusDeviceError::Cs)?;

=======
impl<'d, M, BUS, CS, BusErr, CsErr> embedded_hal_02::blocking::spi::Transfer<u8> for SpiBusDevice<'_, M, BUS, CS>
where
    M: RawMutex,
    BUS: embedded_hal_02::blocking::spi::Transfer<u8, Error = BusErr>,
    CS: OutputPin<Error = CsErr>,
{
    type Error = SpiBusDeviceError<BusErr, CsErr>;
    fn transfer<'w>(&mut self, words: &'w mut [u8]) -> Result<&'w [u8], Self::Error> {
        self.bus.lock(|bus| {
            let mut bus = bus.borrow_mut();
            self.cs.set_low().map_err(SpiBusDeviceError::Cs)?;
            let f_res = bus.transfer(words);
            let cs_res = self.cs.set_high();
            let f_res = f_res.map_err(SpiBusDeviceError::Spi)?;
            cs_res.map_err(SpiBusDeviceError::Cs)?;
            Ok(f_res)
        })
    }
}

impl<'d, M, BUS, CS, BusErr, CsErr> embedded_hal_02::blocking::spi::Write<u8> for SpiBusDevice<'_, M, BUS, CS>
where
    M: RawMutex,
    BUS: embedded_hal_02::blocking::spi::Write<u8, Error = BusErr>,
    CS: OutputPin<Error = CsErr>,
{
    type Error = SpiBusDeviceError<BusErr, CsErr>;

    fn write(&mut self, words: &[u8]) -> Result<(), Self::Error> {
        self.bus.lock(|bus| {
            let mut bus = bus.borrow_mut();
            self.cs.set_low().map_err(SpiBusDeviceError::Cs)?;
            let f_res = bus.write(words);
            let cs_res = self.cs.set_high();
            let f_res = f_res.map_err(SpiBusDeviceError::Spi)?;
            cs_res.map_err(SpiBusDeviceError::Cs)?;
>>>>>>> master
            Ok(f_res)
        })
    }
}
