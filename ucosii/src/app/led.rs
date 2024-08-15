use stm32_metapac::{self, gpio::vals, GPIOA, RCC};

/// init the LED
#[allow(dead_code)]
pub fn LED_Init(){
    // enable the RCC
    RCC.ahb1enr().modify(|v|{
        v.set_gpioaen(true);
    });
    // set GPIO 
    GPIOA.moder().modify(|v|{
        // set mode as output
        v.set_moder(5, vals::Moder::OUTPUT);
    });
    GPIOA.otyper().modify(|v|{
        // set output type as push-pull
        v.set_ot(5, vals::Ot::PUSHPULL);
    });
    GPIOA.ospeedr().modify(|v|{
        // set output speed as high
        v.set_ospeedr(5, vals::Ospeedr::HIGHSPEED);
    });
    GPIOA.pupdr().modify(|v|{
        // set pull-up/pull-down as no pull-up/pull-down
        v.set_pupdr(5, vals::Pupdr::FLOATING);
    });
    GPIOA.odr().modify(|v|{
        // set output as high
        v.set_odr(5, vals::Odr::HIGH);
    });
}

/// turn on the LED
#[allow(dead_code)]
#[inline]
pub fn LED_ON(){
    GPIOA.odr().modify(|v|{
        v.set_odr(5, vals::Odr::HIGH);
    });
}

/// turn off the LED
#[allow(dead_code)]
#[inline]
pub fn LED_OFF(){
    GPIOA.odr().modify(|v|{
        v.set_odr(5, vals::Odr::LOW);
    });
}