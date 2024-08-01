#![no_std]
#![no_main]

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![feature(allocator_api)]
#![feature(alloc_layout_extra)]
#![feature(slice_ptr_get)]
#![feature(sync_unsafe_cell)]
#![feature(alloc_error_handler)]
#![feature(const_mut_refs)]
#![warn(missing_docs)]
//! the mod of uC/OS-II kernel and the interface that uC/OS-II kernel provides
/// This mod MUST go first, so that the others see its macros.
/*
********************************************************************************************************************************************
*                                                               pub mod
********************************************************************************************************************************************
*/

use cortex_m::{interrupt, register::primask};
use critical_section::{set_impl, Impl, RawRestoreState};
use defmt_rtt as _; // global logger

extern crate alloc;
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
// /// the mod of task of uC/OS-II kernel
// pub mod os_task;
/// the mod of semaphore of uC/OS-II kernel
pub mod os_sem;
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
/// the async scheduler(executor) of rust uC
pub mod executor;
/// the stk allocator
pub mod arena;
/// the task interface of uC/OS-II kernel
pub mod os_task;
/// the the macro of atomic operation
#[macro_use]
/// the atomic_macros module is used to define atomic operations
pub mod atomic_macros;

mod heap;

mod util;
/// the mod of lang_items
pub mod lang_items;


/*
********************************************************************************************************************************************
*                                                               critical section
********************************************************************************************************************************************
*/


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
*                                                               type define
********************************************************************************************************************************************
*/
// /// address is a raw pointer
// pub type Addr = *mut core::ffi::c_void;
// /// Unsigned  8 bit quantity
// pub type VoidPtr = *mut core::ffi::c_void;
#[cfg(test)]
#[defmt_test::tests]
mod unit_tests {
    use defmt::assert;
    #[test]
    fn it_works() {
        assert!(true)
    }
}