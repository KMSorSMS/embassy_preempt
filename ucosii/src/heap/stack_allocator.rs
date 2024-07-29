/*
********************************************************************************************************************************************
*                                                               type define
********************************************************************************************************************************************
*/

use alloc::alloc::{GlobalAlloc, Layout};
use core::ptr::NonNull;

use super::fixed_size_block::FixedSizeBlockAllocator;
use super::Locked;

pub const STACK_START: usize = 0x08002000;
pub const STACK_SIZE: usize = 100 * 1024; // 100 KiB

use crate::port::OS_STK;
static ALLOCATOR: Locked<FixedSizeBlockAllocator> = Locked::new(FixedSizeBlockAllocator::new());

pub fn init_stack_allocator() {
    unsafe {
        ALLOCATOR.lock().init(STACK_START, STACK_SIZE);
    }
}
/// alloc a new stack
pub fn alloc_stack(layout: Layout) -> OS_STK_REF {
    unsafe { stk_from_ptr(ALLOCATOR.alloc(layout) as *mut OS_STK,layout) }
}
/// dealloc a stack
pub fn dealloc_stack(stk: OS_STK_REF) {
    unsafe {
        ALLOCATOR.dealloc(stk.as_ptr() as *mut u8, stk.layout);
    }
}

/// the ref of the stk
pub struct OS_STK_REF {
    /// the ref of the stk(top or bottom)
    pub STK_REF: NonNull<OS_STK>,
    /// the layout(size) of the stk
    pub layout: Layout,
}

/// when the OS_STK_REF is default, we will not alloc a stack
impl Default for OS_STK_REF {
    fn default() -> Self {
        OS_STK_REF {
            STK_REF: NonNull::dangling(),
            layout:  Layout::from_size_align(0, 1).unwrap(),
        }
    }
}
/// we impl drop for OS_STK_REF to dealloc the stack(try to be RAII)
impl Drop for OS_STK_REF {
    fn drop(&mut self) {
        if self.STK_REF.as_ptr().is_null() {
            return;
        }
        unsafe {
            ALLOCATOR.dealloc(self.STK_REF.as_ptr() as *mut u8, self.layout);
        }
    }
}

impl OS_STK_REF {
    pub fn as_ptr(&self) -> *mut OS_STK {
        self.STK_REF.as_ptr()
    }
}

pub fn stk_from_ptr(stk_ptr: *mut OS_STK, layout: Layout) -> OS_STK_REF {
    OS_STK_REF {
        STK_REF: NonNull::new(stk_ptr).unwrap(),
        layout,
    }
}
