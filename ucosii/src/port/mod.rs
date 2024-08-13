#![allow(non_camel_case_types)]
use stm32_metapac::timer::TimGp16;

/*
**************************************************************************************************************************************
*                                                               type define
*                                           this part needs to change according to the platform
**************************************************************************************************************************************
*/

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
#[cfg(feature="time_driver_tim1")]
const TIMER:TimGp16 = stm32_metapac::TIM1;
#[cfg(feature="time_driver_tim2")]
const TIMER:TimGp32 = stm32_metapac::TIM2;
#[cfg(feature="time_driver_tim3")]
// by noah: in current project, we use Timer 3 as the time driver
pub const TIMER:TimGp16 = stm32_metapac::TIM3;
#[cfg(feature="time_driver_tim4")]
const TIMER:TimGp16 = stm32_metapac::TIM4;
#[cfg(feature="time_driver_tim5")]
const TIMER:TimGp32 = stm32_metapac::TIM5;
#[cfg(feature="time_driver_tim8")]
const TIMER:TimGp16 = stm32_metapac::TIM8;
#[cfg(feature="time_driver_tim9")]
const TIMER:TimGp16 = stm32_metapac::TIM9;
#[cfg(feature="time_driver_tim12")]
const TIMER:TimGp16 = stm32_metapac::TIM12;
#[cfg(feature="time_driver_tim15")]
const TIMER:TimGp16 = stm32_metapac::TIM15;
#[cfg(feature="time_driver_tim20")]
const TIMER:TimGp16 = stm32_metapac::TIM20;
#[cfg(feature="time_driver_tim21")]
const TIMER:TimGp16 = stm32_metapac::TIM21;
#[cfg(feature="time_driver_tim22")]
const TIMER:TimGp16 = stm32_metapac::TIM22;
#[cfg(feature="time_driver_tim23")]
const TIMER:TimGp16 = stm32_metapac::TIM23;
#[cfg(feature="time_driver_tim24")]
const TIMER:TimGp16 = stm32_metapac::TIM24;


pub mod os_cpu;
/// the time driver
pub mod time_driver;