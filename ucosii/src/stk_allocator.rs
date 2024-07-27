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

use core::{alloc::Layout, cell::{RefCell, UnsafeCell}, mem::MaybeUninit, ptr::{null_mut, NonNull}};
use critical_section::{CriticalSection, Mutex};

use crate::{cfg::OS_ARENA_SIZE, port::*};

/*
********************************************************************************************************************************************
*                                                               type define
********************************************************************************************************************************************
*/

/// the ref of the stk
pub struct OS_STK_REF{
    /// the ref of the stk(top or bottom)
    pub STK_REF:NonNull<OS_STK>,
}

/*
********************************************************************************************************************************************
*                                                              stack allocator
*                                      The stack allocator of uC/OS-II. There are two main functions of it 
*                                      1. alloc the stack memory for the task(TCB) list.
*                                      2. alloc the stack when a future is interrupted without await.
********************************************************************************************************************************************
*/
/// Every TCB(here, we store TaskStorage) will be stored here.
pub static ARENA: Arena<{OS_ARENA_SIZE}> = Arena::new();

/// The stack allocator defination of uC/OS-II.
pub struct Arena<const N: usize> {
    buf: UnsafeCell<MaybeUninit<[u8; N]>>,
    ptr: Mutex<RefCell<*mut u8>>,
}

/*
********************************************************************************************************************************************
*                                                         implements of stack allocator
********************************************************************************************************************************************
*/
unsafe impl<const N: usize> Sync for Arena<N> {}
unsafe impl<const N: usize> Send for Arena<N> {}

// function one：alloc the stack memory for the task(TCB) list.
impl<const N: usize> Arena<N> {
    const fn new() -> Self {
        Self {
            buf: UnsafeCell::new(MaybeUninit::uninit()),
            ptr: Mutex::new(RefCell::new(null_mut())),
        }
    }

    /// this function is used to alloc an area. It will be called in other crate
    pub fn alloc<T>(&'static self, cs: CriticalSection) -> &'static mut MaybeUninit<T> {
        let layout = Layout::new::<T>();

        let start = self.buf.get().cast::<u8>();
        let end = unsafe { start.add(N) };

        let mut ptr = self.ptr.borrow_ref_mut(cs);
        if ptr.is_null() {
            *ptr = self.buf.get().cast::<u8>();
        }

        let bytes_left = (end as usize) - (*ptr as usize);
        let align_offset = (*ptr as usize).next_multiple_of(layout.align()) - (*ptr as usize);

        if align_offset + layout.size() > bytes_left {
            panic!("task arena is full. You must increase the arena size(OS_ARENA_SIZE in cfg)");
        }

        let res = unsafe { ptr.add(align_offset) };
        let ptr = unsafe { ptr.add(align_offset + layout.size()) };

        *(self.ptr.borrow_ref_mut(cs)) = ptr;

        unsafe { &mut *(res as *mut MaybeUninit<T>) }
    }
}