use stm32_metapac::gpio::vals;
use stm32_metapac::{self, GPIOA, RCC};

/// init the LED
#[allow(dead_code)]
pub fn LED_Init() {
    // enable the RCC
    RCC.ahb1enr().modify(|v| {
        v.set_gpioaen(true);
    });
    // set GPIO
    GPIOA.moder().modify(|v| {
        // set mode as output
        v.set_moder(5, vals::Moder::OUTPUT);
    });
    GPIOA.otyper().modify(|v| {
        // set output type as push-pull
        v.set_ot(5, vals::Ot::PUSHPULL);
    });
    GPIOA.ospeedr().modify(|v| {
        // set output speed as high
        v.set_ospeedr(5, vals::Ospeedr::HIGHSPEED);
    });
    GPIOA.pupdr().modify(|v| {
        // set pull-up/pull-down as no pull-up/pull-down
        v.set_pupdr(5, vals::Pupdr::FLOATING);
    });
    GPIOA.odr().modify(|v| {
        // set output as high
        v.set_odr(5, vals::Odr::HIGH);
    });
}

/// turn on the LED
#[allow(dead_code)]
#[inline]
pub fn LED_ON() {
    GPIOA.odr().modify(|v| {
        v.set_odr(5, vals::Odr::HIGH);
    });
}

/// turn off the LED
#[allow(dead_code)]
#[inline]
pub fn LED_OFF() {
    GPIOA.odr().modify(|v| {
        v.set_odr(5, vals::Odr::LOW);
    });
}

/// TEST: thread pin and interrupt pin are used in the time_performance test
/// use the PA0 as the thread pin
/// use the PA1 as the interrupt pin
/// use the PA4 as the stack pin
#[allow(dead_code)]
pub fn Pin_Init(){
    // enable the RCC
    RCC.ahb1enr().modify(|v| {
        v.set_gpioaen(true);
    });
    // set GPIO
    GPIOA.moder().modify(|v| {
        // set mode as output
        v.set_moder(0, vals::Moder::OUTPUT);
        v.set_moder(1, vals::Moder::OUTPUT);
        v.set_moder(4, vals::Moder::OUTPUT);
    });
    GPIOA.otyper().modify(|v| {
        // set output type as push-pull
        v.set_ot(0, vals::Ot::PUSHPULL);
        v.set_ot(1, vals::Ot::PUSHPULL);
        v.set_ot(4, vals::Ot::PUSHPULL);
    });
    GPIOA.ospeedr().modify(|v| {
        // set output speed as high
        v.set_ospeedr(0, vals::Ospeedr::HIGHSPEED);
        v.set_ospeedr(1, vals::Ospeedr::HIGHSPEED);
        v.set_ospeedr(4, vals::Ospeedr::HIGHSPEED);
    });
    GPIOA.pupdr().modify(|v| {
        // set pull-up/pull-down as no pull-up/pull-down
        v.set_pupdr(0, vals::Pupdr::FLOATING);
        v.set_pupdr(1, vals::Pupdr::FLOATING);
        v.set_pupdr(4, vals::Pupdr::FLOATING);
    });
    GPIOA.odr().modify(|v| {
        // set output as low
        v.set_odr(0, vals::Odr::LOW);
        v.set_odr(1, vals::Odr::LOW);
        v.set_odr(4, vals::Odr::LOW);
    });
}

/// set the thread pin high
#[allow(dead_code)]
#[inline]
pub fn thread_pin_high() {
    GPIOA.odr().modify(|v| {
        v.set_odr(0, vals::Odr::HIGH);
    });
}

/// set the thread pin low
#[allow(dead_code)]
#[inline]
pub fn thread_pin_low() {
    GPIOA.odr().modify(|v| {
        v.set_odr(0, vals::Odr::LOW);
    });
}

/// set the interrupt pin high
#[allow(dead_code)]
#[inline]
pub fn interrupt_pin_high() {
    GPIOA.odr().modify(|v| {
        v.set_odr(1, vals::Odr::HIGH);
    });
}

/// set the interrupt pin low
#[allow(dead_code)]
#[inline]
pub fn interrupt_pin_low() {
    GPIOA.odr().modify(|v| {
        v.set_odr(1, vals::Odr::LOW);
    });
}

/// set the stack pin high
#[allow(dead_code)]
#[inline]
pub fn stack_pin_high() {
    GPIOA.odr().modify(|v| {
        v.set_odr(4, vals::Odr::HIGH);
    });
}

/// set the stack pin low
#[allow(dead_code)]
#[inline]
pub fn stack_pin_low() {
    GPIOA.odr().modify(|v| {
        v.set_odr(4, vals::Odr::LOW);
    });
}