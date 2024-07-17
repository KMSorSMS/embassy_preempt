#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![feature(allocator_api)]
#![feature(alloc_layout_extra)]
#![feature(slice_ptr_get)]
#![feature(sync_unsafe_cell)]
#![feature(alloc_error_handler)]
#![warn(missing_docs)]
//! the mod of uC/OS-II kernel and the interface that uC/OS-II kernel provides
// #[macro_use]
// extern crate lazy_static;
use ucosii_macros::heap;
/// This mod MUST go first, so that the others see its macros.
extern crate alloc;
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

#[macro_use]
/// the atomic_macros module is used to define atomic operations
pub mod atomic_macros;
pub mod helper;

mod heap;

mod util;
mod platform;
mod lang_items;

/// Re-exports for use inside macros.
#[doc(hidden)]
pub mod _rt {
    pub use ::core;
    pub use crate::helper;
}

heap! {
    // Heap configuration key in `Drone.toml`.
    config => main;
    /// The main heap allocator generated from the `Drone.toml`.
    metadata => pub Heap;
    // Use this heap as the global allocator.
    global => true;
    // Uncomment the following line to enable heap tracing feature:
    // trace_port => 31;
}

/// The global allocator.
#[cfg_attr(not(feature = "std"), global_allocator)]
pub static HEAP: Heap = Heap::new();
/*
*********************************************************************************
*                                  type define
*********************************************************************************
*/
/// address is a raw pointer
pub type Addr = *mut core::ffi::c_void;
/// Unsigned  8 bit quantity
pub type VoidPtr = *mut core::ffi::c_void;