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
pub type PTR = u32;
/// the usize type used in array
pub type USIZE = usize;


/// Each stack entry is 32-bit wide
pub type OS_STK = usize;
/// Define size of CPU status register (PSR = 32 bits)
pub type OS_CPU_SR = u32;


pub mod os_cpu;
