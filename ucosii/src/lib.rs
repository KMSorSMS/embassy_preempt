#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused)]
// #![warn(missing_docs)]
//! the mod of uC/OS-II kernel and the interface that uC/OS-II kernel provides
// #[macro_use]
// extern crate lazy_static;
/// This mod MUST go first, so that the others see its macros.
pub(crate) mod fmt;

/// the mod of uC/OS-II kernel
pub mod os_core;
/// the mod of flag of uC/OS-II kernel
pub mod os_flag;
/// the mod of mailbox of uC/OS-II kernel
pub mod os_mbox;
/// the mod of memory management of uC/OS-II kernel
pub mod os_mem;
/// the mod of mutex of uC/OS-II kernel
pub mod os_mutex;
/// the mod of queue of uC/OS-II kernel
pub mod os_q;
/// the mod of semaphore of uC/OS-II kernel
pub mod os_sem;
/// the mod of the task part of uC/OS-II kernel
pub mod os_task;
/// the mod of time of uC/OS-II kernel
pub mod os_time;
/// the mod of timer of uC/OS-II kernel
pub mod os_tmr;
/// need to import port here
pub mod port;
/// need to import port here
pub mod cfg;
/// the mod which define the data structure of uC/OS-II kernel
pub mod ucosii;

mod util;

/*
*********************************************************************************
*                                  type define
*********************************************************************************
*/
/// address is a raw pointer
pub type Addr = *mut core::ffi::c_void;
pub type VoidPtr = *mut core::ffi::c_void;