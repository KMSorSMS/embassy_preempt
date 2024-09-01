#![allow(non_camel_case_types)]
use cortex_m::peripheral::scb::SystemHandler;
use stm32_metapac::timer::TimGp16;
#[cfg(feature = "defmt")]
#[allow(unused_imports)]
use defmt::{info,trace};

/*
**************************************************************************************************************************************
*                                                               type define
*                                           this part needs to change according to the platform
**************************************************************************************************************************************
*/
/// ENABLE
pub const ENABLE:bool = true;
/// DISABLE
pub const DISENABLE:bool = false;
/// Unsigned  8 bit quantity
pub type BOOLEAN = bool;
/// Unsigned  8 bit quantity  
pub type INT8U = u8;
/// Signed    8 bit quantity
pub type INT8S = i8;
/// Unsigned 16 bit quantity
pub type INT16U = u16;
/// Signed   16 bit quantity
pub type INT16S = i16;
/// Unsigned 32 bit quantity
pub type INT32U = u32;
/// Signed   32 bit quantity
pub type INT32S = i32;
/// Single precision floating point
pub type FP32 = f32;
/// Double precision floating point
pub type FP64 = f64;
/// the ptr size. define this to use raw ptr
pub type PTR = *mut ();
/// the usize type used in array
pub type USIZE = usize;
/// the u64 type
pub type INT64U = u64;
/// Each stack entry is 32-bit wide
pub type OS_STK = usize;
/// Define size of CPU status register (PSR = 32 bits)
pub type OS_CPU_SR = u32;
/// the timer used as the time Driver
#[cfg(feature = "time_driver_tim1")]
const TIMER: TimGp16 = stm32_metapac::TIM1;
#[cfg(feature = "time_driver_tim2")]
const TIMER: TimGp32 = stm32_metapac::TIM2;
#[cfg(feature = "time_driver_tim3")]
// by noah: in current project, we use Timer 3 as the time driver
/// set the time driver to be Timer3
pub const TIMER: TimGp16 = stm32_metapac::TIM3;
#[cfg(feature = "time_driver_tim4")]
const TIMER: TimGp16 = stm32_metapac::TIM4;
#[cfg(feature = "time_driver_tim5")]
const TIMER: TimGp32 = stm32_metapac::TIM5;
#[cfg(feature = "time_driver_tim8")]
const TIMER: TimGp16 = stm32_metapac::TIM8;
#[cfg(feature = "time_driver_tim9")]
const TIMER: TimGp16 = stm32_metapac::TIM9;
#[cfg(feature = "time_driver_tim12")]
const TIMER: TimGp16 = stm32_metapac::TIM12;
#[cfg(feature = "time_driver_tim15")]
const TIMER: TimGp16 = stm32_metapac::TIM15;
#[cfg(feature = "time_driver_tim20")]
const TIMER: TimGp16 = stm32_metapac::TIM20;
#[cfg(feature = "time_driver_tim21")]
const TIMER: TimGp16 = stm32_metapac::TIM21;
#[cfg(feature = "time_driver_tim22")]
const TIMER: TimGp16 = stm32_metapac::TIM22;
#[cfg(feature = "time_driver_tim23")]
const TIMER: TimGp16 = stm32_metapac::TIM23;
#[cfg(feature = "time_driver_tim24")]
const TIMER: TimGp16 = stm32_metapac::TIM24;

///the language items
pub mod lang_items;
pub mod os_cpu;
/// the time driver
pub mod time_driver;

/// the bottom driver
pub mod bottom_driver;
/*
********************************************************************************************************************************************
*                                                               critical section
********************************************************************************************************************************************
*/
use cortex_m::{interrupt, Peripherals};
use cortex_m::register::primask;
use critical_section::{set_impl, Impl, RawRestoreState};

struct SingleCoreCriticalSection;
set_impl!(SingleCoreCriticalSection);

unsafe impl Impl for SingleCoreCriticalSection {
    unsafe fn acquire() -> RawRestoreState {
        let was_active = primask::read().is_active();
        interrupt::disable();
        was_active
    }

    unsafe fn release(was_active: RawRestoreState) {
        // Only re-enable interrupts if they were enabled before the critical section.
        if was_active {
            interrupt::enable()
        }
    }
}

/*
********************************************************************************************************************************************
*                                                          core peripherals init
********************************************************************************************************************************************
*/

/// by noah: init the core peripherals. For the task() just can be called **once**, we should init the core peripherals together
pub fn init_core_peripherals() {
    let mut p = Peripherals::take().unwrap();
    // set the NVIC
    unsafe{
        // set the group as 2-2
        let aircr = p.SCB.aircr.read();
        let mut aircr = aircr & !(0b1111 << 8);
        aircr = aircr | (0b101 << 8);
        p.SCB.aircr.write(aircr);
        // infer that the group is 2-2
        // set the TIM3 prio as 3
        #[cfg(feature = "defmt")]
        info!("the prio of TIM3 is {}",NVIC::get_priority(stm32_metapac::Interrupt::TIM3));
        p.NVIC.set_priority(stm32_metapac::Interrupt::TIM3, 32);
        #[cfg(feature = "defmt")]
        info!("the prio of TIM3 is {}",NVIC::get_priority(stm32_metapac::Interrupt::TIM3));

        #[cfg(feature = "defmt")]
        info!("the prio of EXTI15_10 is {}",NVIC::get_priority(stm32_metapac::Interrupt::EXTI15_10));
        // set the EXTI13 prio as 1
        p.NVIC.set_priority(stm32_metapac::Interrupt::EXTI15_10, 16);
        #[cfg(feature = "defmt")]
        info!("the prio of EXTI15_10 is {}",NVIC::get_priority(stm32_metapac::Interrupt::EXTI15_10));
        #[cfg(feature = "defmt")]
        info!("the prio of PendSV is {}",SCB::get_priority(SystemHandler::PendSV));
        p.SCB.set_priority(SystemHandler::PendSV, 0xf<<4);
        #[cfg(feature = "defmt")]
        info!("the prio of PendSV is {}",SCB::get_priority(SystemHandler::PendSV));
    }
}