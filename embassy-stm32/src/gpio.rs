#![macro_use]
use core::convert::Infallible;

use embassy_hal_common::{impl_unborrow, unborrow, Unborrowed};

use crate::pac::gpio::{self, vals};
use crate::{pac, peripherals, Unborrow};

/// GPIO flexible pin.
///
/// This pin can either be a disconnected, input, or output pin, or both. The level register bit will remain
/// set while not in output mode, so the pin's level will be 'remembered' when it is not in output
/// mode.
pub struct Flex<'d, T: Pin> {
    pub(crate) pin: Unborrowed<'d, T>,
}

impl<'d, T: Pin> Flex<'d, T> {
    /// Wrap the pin in a `Flex`.
    ///
    /// The pin remains disconnected. The initial output level is unspecified, but can be changed
    /// before the pin is put into output mode.
    ///
    #[inline]
    pub fn new(pin: impl Unborrow<Target = T> + 'd) -> Self {
        unborrow!(pin);
        // Pin will be in disconnected state.
        Self { pin }
    }

    /// Put the pin into input mode.
    #[inline]
    pub fn set_as_input(&mut self, pull: Pull) {
        critical_section::with(|_| unsafe {
            let r = self.pin.block();
            let n = self.pin.pin() as usize;
            #[cfg(gpio_v1)]
            {
                let cnf = match pull {
                    Pull::Up => {
                        r.bsrr().write(|w| w.set_bs(n, true));
                        vals::CnfIn::PULL
                    }
                    Pull::Down => {
                        r.bsrr().write(|w| w.set_br(n, true));
                        vals::CnfIn::PULL
                    }
                    Pull::None => vals::CnfIn::FLOATING,
                };

                let crlh = if n < 8 { 0 } else { 1 };
                r.cr(crlh).modify(|w| {
                    w.set_mode(n % 8, vals::Mode::INPUT);
                    w.set_cnf_in(n % 8, cnf);
                });
            }
            #[cfg(gpio_v2)]
            {
                r.pupdr().modify(|w| w.set_pupdr(n, pull.into()));
                r.otyper().modify(|w| w.set_ot(n, vals::Ot::PUSHPULL));
                r.moder().modify(|w| w.set_moder(n, vals::Moder::INPUT));
            }
        });
    }

    /// Put the pin into output mode.
    ///
    /// The pin level will be whatever was set before (or low by default). If you want it to begin
    /// at a specific level, call `set_high`/`set_low` on the pin first.
    #[inline]
    pub fn set_as_output(&mut self, speed: Speed) {
        critical_section::with(|_| unsafe {
            let r = self.pin.block();
            let n = self.pin.pin() as usize;
            #[cfg(gpio_v1)]
            {
                let crlh = if n < 8 { 0 } else { 1 };
                r.cr(crlh).modify(|w| {
                    w.set_mode(n % 8, speed.into());
                    w.set_cnf_out(n % 8, vals::CnfOut::PUSHPULL);
                });
            }
            #[cfg(gpio_v2)]
            {
                r.pupdr().modify(|w| w.set_pupdr(n, vals::Pupdr::FLOATING));
                r.otyper().modify(|w| w.set_ot(n, vals::Ot::PUSHPULL));
                self.pin.set_speed(speed);
                r.moder().modify(|w| w.set_moder(n, vals::Moder::OUTPUT));
            }
        });
    }

    /// Put the pin into input + output mode.
    ///
    /// This is commonly used for "open drain" mode.
    /// the hardware will drive the line low if you set it to low, and will leave it floating if you set
    /// it to high, in which case you can read the input to figure out whether another device
    /// is driving the line low.
    ///
    /// The pin level will be whatever was set before (or low by default). If you want it to begin
    /// at a specific level, call `set_high`/`set_low` on the pin first.
    #[inline]
    pub fn set_as_input_output(&mut self, speed: Speed, pull: Pull) {
        critical_section::with(|_| unsafe {
            let r = self.pin.block();
            let n = self.pin.pin() as usize;
            #[cfg(gpio_v1)]
            {
                let crlh = if n < 8 { 0 } else { 1 };
                match pull {
                    Pull::Up => r.bsrr().write(|w| w.set_bs(n, true)),
                    Pull::Down => r.bsrr().write(|w| w.set_br(n, true)),
                    Pull::None => {}
                }
                r.cr(crlh).modify(|w| w.set_mode(n % 8, speed.into()));
                r.cr(crlh).modify(|w| w.set_cnf_out(n % 8, vals::CnfOut::OPENDRAIN));
            }
            #[cfg(gpio_v2)]
            {
                r.pupdr().modify(|w| w.set_pupdr(n, pull.into()));
                r.otyper().modify(|w| w.set_ot(n, vals::Ot::OPENDRAIN));
                self.pin.set_speed(speed);
                r.moder().modify(|w| w.set_moder(n, vals::Moder::OUTPUT));
            }
        });
    }

    #[inline]
    pub fn is_high(&self) -> bool {
        !self.is_low()
    }

    #[inline]
    pub fn is_low(&self) -> bool {
        let state = unsafe { self.pin.block().idr().read().idr(self.pin.pin() as _) };
        state == vals::Idr::LOW
    }

    #[inline]
    pub fn get_level(&self) -> Level {
        self.is_high().into()
    }

    #[inline]
    pub fn is_set_high(&self) -> bool {
        !self.is_set_low()
    }

    /// Is the output pin set as low?
    #[inline]
    pub fn is_set_low(&self) -> bool {
        let state = unsafe { self.pin.block().odr().read().odr(self.pin.pin() as _) };
        state == vals::Odr::LOW
    }

    /// What level output is set to
    #[inline]
    pub fn get_output_level(&self) -> Level {
        self.is_set_high().into()
    }

    #[inline]
    pub fn set_high(&mut self) {
        self.pin.set_high();
    }

    /// Set the output as low.
    #[inline]
    pub fn set_low(&mut self) {
        self.pin.set_low();
    }

    #[inline]
    pub fn set_level(&mut self, level: Level) {
        match level {
            Level::Low => self.pin.set_low(),
            Level::High => self.pin.set_high(),
        }
    }

    /// Toggle pin output
    #[inline]
    pub fn toggle(&mut self) {
        if self.is_set_low() {
            self.set_high()
        } else {
            self.set_low()
        }
    }
}

impl<'d, T: Pin> Drop for Flex<'d, T> {
    #[inline]
    fn drop(&mut self) {
        critical_section::with(|_| unsafe {
            let r = self.pin.block();
            let n = self.pin.pin() as usize;
            #[cfg(gpio_v1)]
            {
                let crlh = if n < 8 { 0 } else { 1 };
                r.cr(crlh).modify(|w| {
                    w.set_mode(n % 8, vals::Mode::INPUT);
                    w.set_cnf_in(n % 8, vals::CnfIn::FLOATING);
                });
            }
            #[cfg(gpio_v2)]
            {
                r.pupdr().modify(|w| w.set_pupdr(n, vals::Pupdr::FLOATING));
                r.moder().modify(|w| w.set_moder(n, vals::Moder::INPUT));
            }
        });
    }
}

/// Pull setting for an input.
#[derive(Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pull {
    None,
    Up,
    Down,
}

#[cfg(gpio_v2)]
impl From<Pull> for vals::Pupdr {
    fn from(pull: Pull) -> Self {
        use Pull::*;

        match pull {
            None => vals::Pupdr::FLOATING,
            Up => vals::Pupdr::PULLUP,
            Down => vals::Pupdr::PULLDOWN,
        }
    }
}

/// Speed settings
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Speed {
    Low,
    Medium,
    #[cfg(not(any(syscfg_f0, gpio_v1)))]
    High,
    VeryHigh,
}

#[cfg(gpio_v1)]
impl From<Speed> for vals::Mode {
    fn from(speed: Speed) -> Self {
        use Speed::*;

        match speed {
            Low => vals::Mode::OUTPUT2MHZ,
            Medium => vals::Mode::OUTPUT10MHZ,
            VeryHigh => vals::Mode::OUTPUT50MHZ,
        }
    }
}

#[cfg(gpio_v2)]
impl From<Speed> for vals::Ospeedr {
    fn from(speed: Speed) -> Self {
        use Speed::*;

        match speed {
            Low => vals::Ospeedr::LOWSPEED,
            Medium => vals::Ospeedr::MEDIUMSPEED,
            #[cfg(not(syscfg_f0))]
            High => vals::Ospeedr::HIGHSPEED,
            VeryHigh => vals::Ospeedr::VERYHIGHSPEED,
        }
    }
}

/// GPIO input driver.
pub struct Input<'d, T: Pin> {
    pub(crate) pin: Flex<'d, T>,
}

impl<'d, T: Pin> Input<'d, T> {
    #[inline]
    pub fn new(pin: impl Unborrow<Target = T> + 'd, pull: Pull) -> Self {
        let mut pin = Flex::new(pin);
        pin.set_as_input(pull);
        Self { pin }
    }

    #[inline]
    pub fn is_high(&self) -> bool {
        self.pin.is_high()
    }

    #[inline]
    pub fn is_low(&self) -> bool {
        self.pin.is_low()
    }

    #[inline]
    pub fn get_level(&self) -> Level {
        self.pin.get_level()
    }
}

/// Digital input or output level.
#[derive(Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Level {
    Low,
    High,
}

impl From<bool> for Level {
    fn from(val: bool) -> Self {
        match val {
            true => Self::High,
            false => Self::Low,
        }
    }
}

impl Into<bool> for Level {
    fn into(self) -> bool {
        match self {
            Level::Low => false,
            Level::High => true,
        }
    }
}

/// GPIO output driver.
pub struct Output<'d, T: Pin> {
    pub(crate) pin: Flex<'d, T>,
}

impl<'d, T: Pin> Output<'d, T> {
    #[inline]
    pub fn new(pin: impl Unborrow<Target = T> + 'd, initial_output: Level, speed: Speed) -> Self {
        let mut pin = Flex::new(pin);
        match initial_output {
            Level::High => pin.set_high(),
            Level::Low => pin.set_low(),
        }
        pin.set_as_output(speed);
        Self { pin }
    }

    /// Set the output as high.
    #[inline]
    pub fn set_high(&mut self) {
        self.pin.set_high();
    }

    /// Set the output as low.
    #[inline]
    pub fn set_low(&mut self) {
        self.pin.set_low();
    }

    /// Set the output level.
    #[inline]
    pub fn set_level(&mut self, level: Level) {
        self.pin.set_level(level)
    }

    /// Is the output pin set as high?
    #[inline]
    pub fn is_set_high(&self) -> bool {
        self.pin.is_set_high()
    }

    /// Is the output pin set as low?
    #[inline]
    pub fn is_set_low(&self) -> bool {
        self.pin.is_set_low()
    }

    /// What level output is set to
    #[inline]
    pub fn get_output_level(&self) -> Level {
        self.pin.get_output_level()
    }

    /// Toggle pin output
    #[inline]
    pub fn toggle(&mut self) {
        self.pin.toggle();
    }
}

/// GPIO output open-drain driver.
pub struct OutputOpenDrain<'d, T: Pin> {
    pub(crate) pin: Flex<'d, T>,
}

impl<'d, T: Pin> OutputOpenDrain<'d, T> {
    #[inline]
    pub fn new(pin: impl Unborrow<Target = T> + 'd, initial_output: Level, speed: Speed, pull: Pull) -> Self {
        let mut pin = Flex::new(pin);

        match initial_output {
            Level::High => pin.set_high(),
            Level::Low => pin.set_low(),
        }

        pin.set_as_input_output(speed, pull);
        Self { pin }
    }

    #[inline]
    pub fn is_high(&self) -> bool {
        !self.pin.is_low()
    }

    #[inline]
    pub fn is_low(&self) -> bool {
        self.pin.is_low()
    }

    /// Returns current pin level
    #[inline]
    pub fn get_level(&self) -> Level {
        self.pin.get_level()
    }

    /// Set the output as high.
    #[inline]
    pub fn set_high(&mut self) {
        self.pin.set_high();
    }

    /// Set the output as low.
    #[inline]
    pub fn set_low(&mut self) {
        self.pin.set_low();
    }

    /// Set the output level.
    #[inline]
    pub fn set_level(&mut self, level: Level) {
        self.pin.set_level(level);
    }

    /// Is the output pin set as high?
    #[inline]
    pub fn is_set_high(&self) -> bool {
        self.pin.is_set_high()
    }

    /// Is the output pin set as low?
    #[inline]
    pub fn is_set_low(&self) -> bool {
        self.pin.is_set_low()
    }

    /// What level output is set to
    #[inline]
    pub fn get_output_level(&self) -> Level {
        self.pin.get_output_level()
    }

    /// Toggle pin output
    #[inline]
    pub fn toggle(&mut self) {
        self.pin.toggle()
    }
}

pub(crate) mod sealed {
    use super::*;

    /// Alternate function type settings
    #[derive(Debug)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum AFType {
        Input,
        OutputPushPull,
        OutputOpenDrain,
    }

    pub trait Pin {
        fn pin_port(&self) -> u8;

        #[inline]
        fn _pin(&self) -> u8 {
            self.pin_port() % 16
        }
        #[inline]
        fn _port(&self) -> u8 {
            self.pin_port() / 16
        }

        #[inline]
        fn block(&self) -> gpio::Gpio {
            pac::GPIO(self._port() as _)
        }

        /// Set the output as high.
        #[inline]
        fn set_high(&self) {
            unsafe {
                let n = self._pin() as _;
                self.block().bsrr().write(|w| w.set_bs(n, true));
            }
        }

        /// Set the output as low.
        #[inline]
        fn set_low(&self) {
            unsafe {
                let n = self._pin() as _;
                self.block().bsrr().write(|w| w.set_br(n, true));
            }
        }

        #[inline]
        unsafe fn set_as_af(&self, af_num: u8, af_type: AFType) {
            self.set_as_af_pull(af_num, af_type, Pull::None);
        }

        #[cfg(gpio_v1)]
        #[inline]
        unsafe fn set_as_af_pull(&self, _af_num: u8, af_type: AFType, pull: Pull) {
            // F1 uses the AFIO register for remapping.
            // For now, this is not implemented, so af_num is ignored
            // _af_num should be zero here, since it is not set by stm32-data
            let r = self.block();
            let n = self._pin() as usize;
            let crlh = if n < 8 { 0 } else { 1 };
            match af_type {
                AFType::Input => {
                    let cnf = match pull {
                        Pull::Up => {
                            r.bsrr().write(|w| w.set_bs(n, true));
                            vals::CnfIn::PULL
                        }
                        Pull::Down => {
                            r.bsrr().write(|w| w.set_br(n, true));
                            vals::CnfIn::PULL
                        }
                        Pull::None => vals::CnfIn::FLOATING,
                    };

                    r.cr(crlh).modify(|w| {
                        w.set_mode(n % 8, vals::Mode::INPUT);
                        w.set_cnf_in(n % 8, cnf);
                    });
                }
                AFType::OutputPushPull => {
                    r.cr(crlh).modify(|w| {
                        w.set_mode(n % 8, vals::Mode::OUTPUT50MHZ);
                        w.set_cnf_out(n % 8, vals::CnfOut::ALTPUSHPULL);
                    });
                }
                AFType::OutputOpenDrain => {
                    r.cr(crlh).modify(|w| {
                        w.set_mode(n % 8, vals::Mode::OUTPUT50MHZ);
                        w.set_cnf_out(n % 8, vals::CnfOut::ALTOPENDRAIN);
                    });
                }
            }
        }

        #[cfg(gpio_v2)]
        #[inline]
        unsafe fn set_as_af_pull(&self, af_num: u8, af_type: AFType, pull: Pull) {
            let pin = self._pin() as usize;
            let block = self.block();
            block.afr(pin / 8).modify(|w| w.set_afr(pin % 8, af_num));
            match af_type {
                AFType::Input => {}
                AFType::OutputPushPull => block.otyper().modify(|w| w.set_ot(pin, vals::Ot::PUSHPULL)),
                AFType::OutputOpenDrain => block.otyper().modify(|w| w.set_ot(pin, vals::Ot::OPENDRAIN)),
            }
            block.pupdr().modify(|w| w.set_pupdr(pin, pull.into()));

            block.moder().modify(|w| w.set_moder(pin, vals::Moder::ALTERNATE));
        }

        #[inline]
        unsafe fn set_as_analog(&self) {
            let pin = self._pin() as usize;
            let block = self.block();
            #[cfg(gpio_v1)]
            {
                let crlh = if pin < 8 { 0 } else { 1 };
                block.cr(crlh).modify(|w| {
                    w.set_mode(pin % 8, vals::Mode::INPUT);
                    w.set_cnf_in(pin % 8, vals::CnfIn::ANALOG);
                });
            }
            #[cfg(gpio_v2)]
            block.moder().modify(|w| w.set_moder(pin, vals::Moder::ANALOG));
        }

        /// Set the pin as "disconnected", ie doing nothing and consuming the lowest
        /// amount of power possible.
        ///
        /// This is currently the same as set_as_analog but is semantically different really.
        /// Drivers should set_as_disconnected pins when dropped.
        #[inline]
        unsafe fn set_as_disconnected(&self) {
            self.set_as_analog();
        }

        #[inline]
        unsafe fn set_speed(&self, speed: Speed) {
            let pin = self._pin() as usize;

            #[cfg(gpio_v1)]
            {
                let crlh = if pin < 8 { 0 } else { 1 };
                self.block().cr(crlh).modify(|w| {
                    w.set_mode(pin % 8, speed.into());
                });
            }

            #[cfg(gpio_v2)]
            self.block().ospeedr().modify(|w| w.set_ospeedr(pin, speed.into()));
        }
    }
}

pub trait Pin: Into<AnyPin> + sealed::Pin + Sized + 'static {
    #[cfg(feature = "exti")]
    type ExtiChannel: crate::exti::Channel;

    /// Number of the pin within the port (0..31)
    #[inline]
    fn pin(&self) -> u8 {
        self._pin()
    }

    /// Port of the pin
    #[inline]
    fn port(&self) -> u8 {
        self._port()
    }

    /// Convert from concrete pin type PX_XX to type erased `AnyPin`.
    #[inline]
    fn degrade(self) -> AnyPin {
        AnyPin {
            pin_port: self.pin_port(),
        }
    }
}

// Type-erased GPIO pin
pub struct AnyPin {
    pin_port: u8,
}

impl AnyPin {
    #[inline]
    pub unsafe fn steal(pin_port: u8) -> Self {
        Self { pin_port }
    }

    #[inline]
    fn _port(&self) -> u8 {
        self.pin_port / 16
    }

    #[inline]
    pub fn block(&self) -> gpio::Gpio {
        pac::GPIO(self._port() as _)
    }
}

impl_unborrow!(AnyPin);
impl Pin for AnyPin {
    #[cfg(feature = "exti")]
    type ExtiChannel = crate::exti::AnyChannel;
}
impl sealed::Pin for AnyPin {
    #[inline]
    fn pin_port(&self) -> u8 {
        self.pin_port
    }
}

// ====================

foreach_pin!(
    ($pin_name:ident, $port_name:ident, $port_num:expr, $pin_num:expr, $exti_ch:ident) => {
        impl Pin for peripherals::$pin_name {
            #[cfg(feature = "exti")]
            type ExtiChannel = peripherals::$exti_ch;
        }
        impl sealed::Pin for peripherals::$pin_name {
            #[inline]
            fn pin_port(&self) -> u8 {
                $port_num * 16 + $pin_num
            }
        }

        impl From<peripherals::$pin_name> for AnyPin {
            fn from(x: peripherals::$pin_name) -> Self {
                x.degrade()
            }
        }
    };
);

pub(crate) unsafe fn init() {
    crate::_generated::init_gpio();
}

mod eh02 {
    use embedded_hal_02::digital::v2::{InputPin, OutputPin, StatefulOutputPin, ToggleableOutputPin};

    use super::*;

    impl<'d, T: Pin> InputPin for Input<'d, T> {
        type Error = Infallible;

        #[inline]
        fn is_high(&self) -> Result<bool, Self::Error> {
            Ok(self.is_high())
        }

        #[inline]
        fn is_low(&self) -> Result<bool, Self::Error> {
            Ok(self.is_low())
        }
    }

    impl<'d, T: Pin> OutputPin for Output<'d, T> {
        type Error = Infallible;

        #[inline]
        fn set_high(&mut self) -> Result<(), Self::Error> {
            Ok(self.set_high())
        }

        #[inline]
        fn set_low(&mut self) -> Result<(), Self::Error> {
            Ok(self.set_low())
        }
    }

    impl<'d, T: Pin> StatefulOutputPin for Output<'d, T> {
        #[inline]
        fn is_set_high(&self) -> Result<bool, Self::Error> {
            Ok(self.is_set_high())
        }

        /// Is the output pin set as low?
        #[inline]
        fn is_set_low(&self) -> Result<bool, Self::Error> {
            Ok(self.is_set_low())
        }
    }

    impl<'d, T: Pin> ToggleableOutputPin for Output<'d, T> {
        type Error = Infallible;
        #[inline]
        fn toggle(&mut self) -> Result<(), Self::Error> {
            Ok(self.toggle())
        }
    }

    impl<'d, T: Pin> OutputPin for OutputOpenDrain<'d, T> {
        type Error = Infallible;

        #[inline]
        fn set_high(&mut self) -> Result<(), Self::Error> {
            Ok(self.set_high())
        }

        #[inline]
        fn set_low(&mut self) -> Result<(), Self::Error> {
            Ok(self.set_low())
        }
    }

    impl<'d, T: Pin> StatefulOutputPin for OutputOpenDrain<'d, T> {
        #[inline]
        fn is_set_high(&self) -> Result<bool, Self::Error> {
            Ok(self.is_set_high())
        }

        /// Is the output pin set as low?
        #[inline]
        fn is_set_low(&self) -> Result<bool, Self::Error> {
            Ok(self.is_set_low())
        }
    }

    impl<'d, T: Pin> ToggleableOutputPin for OutputOpenDrain<'d, T> {
        type Error = Infallible;
        #[inline]
        fn toggle(&mut self) -> Result<(), Self::Error> {
            Ok(self.toggle())
        }
    }

    impl<'d, T: Pin> InputPin for Flex<'d, T> {
        type Error = Infallible;

        #[inline]
        fn is_high(&self) -> Result<bool, Self::Error> {
            Ok(self.is_high())
        }

        #[inline]
        fn is_low(&self) -> Result<bool, Self::Error> {
            Ok(self.is_low())
        }
    }

    impl<'d, T: Pin> OutputPin for Flex<'d, T> {
        type Error = Infallible;

        #[inline]
        fn set_high(&mut self) -> Result<(), Self::Error> {
            Ok(self.set_high())
        }

        #[inline]
        fn set_low(&mut self) -> Result<(), Self::Error> {
            Ok(self.set_low())
        }
    }

    impl<'d, T: Pin> StatefulOutputPin for Flex<'d, T> {
        #[inline]
        fn is_set_high(&self) -> Result<bool, Self::Error> {
            Ok(self.is_set_high())
        }

        /// Is the output pin set as low?
        #[inline]
        fn is_set_low(&self) -> Result<bool, Self::Error> {
            Ok(self.is_set_low())
        }
    }

    impl<'d, T: Pin> ToggleableOutputPin for Flex<'d, T> {
        type Error = Infallible;
        #[inline]
        fn toggle(&mut self) -> Result<(), Self::Error> {
            Ok(self.toggle())
        }
    }
}

#[cfg(feature = "unstable-traits")]
mod eh1 {
    use embedded_hal_1::digital::blocking::{InputPin, OutputPin, StatefulOutputPin, ToggleableOutputPin};
    use embedded_hal_1::digital::ErrorType;

    use super::*;

    impl<'d, T: Pin> ErrorType for Input<'d, T> {
        type Error = Infallible;
    }

    impl<'d, T: Pin> InputPin for Input<'d, T> {
        #[inline]
        fn is_high(&self) -> Result<bool, Self::Error> {
            Ok(self.is_high())
        }

        #[inline]
        fn is_low(&self) -> Result<bool, Self::Error> {
            Ok(self.is_low())
        }
    }

    impl<'d, T: Pin> ErrorType for Output<'d, T> {
        type Error = Infallible;
    }

    impl<'d, T: Pin> OutputPin for Output<'d, T> {
        #[inline]
        fn set_high(&mut self) -> Result<(), Self::Error> {
            Ok(self.set_high())
        }

        #[inline]
        fn set_low(&mut self) -> Result<(), Self::Error> {
            Ok(self.set_low())
        }
    }

    impl<'d, T: Pin> StatefulOutputPin for Output<'d, T> {
        #[inline]
        fn is_set_high(&self) -> Result<bool, Self::Error> {
            Ok(self.is_set_high())
        }

        /// Is the output pin set as low?
        #[inline]
        fn is_set_low(&self) -> Result<bool, Self::Error> {
            Ok(self.is_set_low())
        }
    }

    impl<'d, T: Pin> ToggleableOutputPin for Output<'d, T> {
        #[inline]
        fn toggle(&mut self) -> Result<(), Self::Error> {
            Ok(self.toggle())
        }
    }

    impl<'d, T: Pin> ErrorType for OutputOpenDrain<'d, T> {
        type Error = Infallible;
    }

    impl<'d, T: Pin> OutputPin for OutputOpenDrain<'d, T> {
        #[inline]
        fn set_high(&mut self) -> Result<(), Self::Error> {
            Ok(self.set_high())
        }

        #[inline]
        fn set_low(&mut self) -> Result<(), Self::Error> {
            Ok(self.set_low())
        }
    }

    impl<'d, T: Pin> StatefulOutputPin for OutputOpenDrain<'d, T> {
        #[inline]
        fn is_set_high(&self) -> Result<bool, Self::Error> {
            Ok(self.is_set_high())
        }

        /// Is the output pin set as low?
        #[inline]
        fn is_set_low(&self) -> Result<bool, Self::Error> {
            Ok(self.is_set_low())
        }
    }

    impl<'d, T: Pin> ToggleableOutputPin for OutputOpenDrain<'d, T> {
        #[inline]
        fn toggle(&mut self) -> Result<(), Self::Error> {
            Ok(self.toggle())
        }
    }

    impl<'d, T: Pin> InputPin for Flex<'d, T> {
        #[inline]
        fn is_high(&self) -> Result<bool, Self::Error> {
            Ok(self.is_high())
        }

        #[inline]
        fn is_low(&self) -> Result<bool, Self::Error> {
            Ok(self.is_low())
        }
    }

    impl<'d, T: Pin> OutputPin for Flex<'d, T> {
        #[inline]
        fn set_high(&mut self) -> Result<(), Self::Error> {
            Ok(self.set_high())
        }

        #[inline]
        fn set_low(&mut self) -> Result<(), Self::Error> {
            Ok(self.set_low())
        }
    }

    impl<'d, T: Pin> ToggleableOutputPin for Flex<'d, T> {
        #[inline]
        fn toggle(&mut self) -> Result<(), Self::Error> {
            Ok(self.toggle())
        }
    }

    impl<'d, T: Pin> ErrorType for Flex<'d, T> {
        type Error = Infallible;
    }

    impl<'d, T: Pin> StatefulOutputPin for Flex<'d, T> {
        #[inline]
        fn is_set_high(&self) -> Result<bool, Self::Error> {
            Ok(self.is_set_high())
        }

        /// Is the output pin set as low?
        #[inline]
        fn is_set_low(&self) -> Result<bool, Self::Error> {
            Ok(self.is_set_low())
        }
    }
}

#[cfg(feature = "unstable-pac")]
pub mod low_level {
    pub use super::sealed::*;
}
