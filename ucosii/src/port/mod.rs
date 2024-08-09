/*
**************************************************************************************************************************************
*                                                               type define
*                                           this part needs to change according to the platform
**************************************************************************************************************************************
*/
#![allow(non_camel_case_types)]

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

/// Each stack entry is 32-bit wide
pub type OS_STK = usize;
/// Define size of CPU status register (PSR = 32 bits)
pub type OS_CPU_SR = u32;
/// the timer used as the time Driver
#[cfg(feature="time_driver_tim1")]
type T = peripherals::TIM1;
#[cfg(feature="time_driver_tim2")]
type T = peripherals::TIM2;
// #[cfg(feature="time_driver_tim3")]
// type T = peripherals::TIM3;
#[cfg(feature="time_driver_tim4")]
type T = peripherals::TIM4;
#[cfg(feature="time_driver_tim5")]
type T = peripherals::TIM5;
#[cfg(feature="time_driver_tim8")]
type T = peripherals::TIM8;
#[cfg(feature="time_driver_tim9")]
type T = peripherals::TIM9;
#[cfg(feature="time_driver_tim12")]
type T = peripherals::TIM12;
#[cfg(feature="time_driver_tim15")]
type T = peripherals::TIM15;
#[cfg(feature="time_driver_tim20")]
type T = peripherals::TIM20;
#[cfg(feature="time_driver_tim21")]
type T = peripherals::TIM21;
#[cfg(feature="time_driver_tim22")]
type T = peripherals::TIM22;
#[cfg(feature="time_driver_tim23")]
type T = peripherals::TIM23;
#[cfg(feature="time_driver_tim24")]
type T = peripherals::TIM24;


pub mod os_cpu;