//! the allocator of the stack this crate only will be used by executor

/*
****************************************************************************************************************************************
*                                                      the stack  of uC/OS-II
*                                                        code by liam and noah
****************************************************************************************************************************************
*/

/*
********************************************************************************************************************************************
*                                                               import mod
********************************************************************************************************************************************
*/

use core::alloc::Layout;
use core::cell::{Cell, UnsafeCell};
use core::mem::MaybeUninit;
use core::ptr::null_mut;

use critical_section::{CriticalSection, Mutex};

use crate::cfg::OS_ARENA_SIZE;

/*
********************************************************************************************************************************************
*                                                              stack allocator
*                                      The stack allocator of uC/OS-II. There are two main functions of it
*                                      1. alloc the stack memory for the task(TCB) list.
*                                      2. alloc the stack when a future is interrupted without await.
********************************************************************************************************************************************
*/
/// Every TCB(here, we store TaskStorage) will be stored here.
pub static ARENA: Arena<{ OS_ARENA_SIZE }> = Arena::new();

/// The stack allocator defination of uC/OS-II.

pub struct Arena<const N: usize> {
    buf: UnsafeCell<MaybeUninit<[u8; N]>>,
    ptr: Mutex<Cell<*mut u8>>,
}
/*
********************************************************************************************************************************************
*                                                         implements of stack allocator
********************************************************************************************************************************************
*/

unsafe impl<const N: usize> Sync for Arena<N> {}
unsafe impl<const N: usize> Send for Arena<N> {}

impl<const N: usize> Arena<N> {
    const fn new() -> Self {
        Self {
            buf: UnsafeCell::new(MaybeUninit::uninit()),
            ptr: Mutex::new(Cell::new(null_mut())),
        }
    }
    /// alloc the stack memory for the task(TCB) list.
    pub fn alloc<T>(&'static self, cs: CriticalSection) -> &'static mut MaybeUninit<T> {
        let layout = Layout::new::<T>();

        let start = self.buf.get().cast::<u8>();
        let end = unsafe { start.add(N) };

        let mut ptr = self.ptr.borrow(cs).get();
        if ptr.is_null() {
            ptr = self.buf.get().cast::<u8>();
        }

        let bytes_left = (end as usize) - (ptr as usize);
        let align_offset = (ptr as usize).next_multiple_of(layout.align()) - (ptr as usize);

        if align_offset + layout.size() > bytes_left {
            panic!("embassy-executor: task arena is full. You must increase the arena size, see the documentation for details: https://docs.embassy.dev/embassy-executor/");
        }
        // info!("size of the task storage is {}", layout.size());
        let res = unsafe { ptr.add(align_offset) };
        let ptr = unsafe { ptr.add(align_offset + layout.size()) };

        self.ptr.borrow(cs).set(ptr);

        unsafe { &mut *(res as *mut MaybeUninit<T>) }
    }
}
