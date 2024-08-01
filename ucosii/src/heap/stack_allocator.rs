/*
********************************************************************************************************************************************
*                                                               type define
********************************************************************************************************************************************
*/

use alloc::alloc::{GlobalAlloc, Layout};
use core::borrow::BorrowMut;
use core::ptr::NonNull;

use super::fixed_size_block::FixedSizeBlockAllocator;
use super::Locked;

pub const STACK_START: usize = 0x08002000;
pub const STACK_SIZE: usize = 100 * 1024; // 100 KiB
pub const PROGRAM_STACK_SIZE: usize = 1024; // 1 KiB
pub const INTERRUPT_STACK_SIZE: usize = 1024; // 1 KiB

use crate::port::OS_STK;
use crate::sync::UPSafeCell;
static STACK_ALLOCATOR: Locked<FixedSizeBlockAllocator> = Locked::new(FixedSizeBlockAllocator::new());
lazy_static::lazy_static! {
pub static ref PROGRAM_STACK: UPSafeCell<OS_STK_REF> = unsafe {
    UPSafeCell::new(OS_STK_REF::default())
};
pub static ref INTERRUPT_STACK: UPSafeCell<OS_STK_REF> = unsafe {
    UPSafeCell::new(OS_STK_REF::default())
};
}

pub fn init_stack_allocator() {
    unsafe {
        STACK_ALLOCATOR.lock().init(STACK_START, STACK_SIZE);
    }
    // then we init the program stack
    let layout = Layout::from_size_align(PROGRAM_STACK_SIZE, 8).unwrap();
    let stk = alloc_stack(layout);
    let mut pr_stk = PROGRAM_STACK.exclusive_access();
    pr_stk.HEAP_REF = stk.HEAP_REF;
    pr_stk.STK_REF = stk.STK_REF;
    pr_stk.layout = stk.layout;
    drop(pr_stk);
    // then we change the sp to the top of the program stack
    // this depending on the arch so we need extern and implement in the port
    extern "Rust" {
        fn set_program_sp(sp: *mut u8);
    }
    unsafe {
        set_program_sp(stk.STK_REF.as_ptr() as *mut u8);
    }
    // we also need to allocate a stack for interrupt
    let layout = Layout::from_size_align(INTERRUPT_STACK_SIZE, 8).unwrap();
    let stk = alloc_stack(layout);
    let mut int_stk = INTERRUPT_STACK.exclusive_access();
    int_stk.HEAP_REF = stk.HEAP_REF;
    int_stk.STK_REF = stk.STK_REF;
    int_stk.layout = stk.layout;
    drop(int_stk);
}
/// alloc a new stack
pub fn alloc_stack(layout: Layout) -> OS_STK_REF {
    unsafe { stk_from_ptr(STACK_ALLOCATOR.alloc(layout), layout) }
}
/// dealloc a stack
pub fn dealloc_stack(stk: &OS_STK_REF) {
    unsafe {
        STACK_ALLOCATOR.dealloc(stk.as_ptr(), stk.layout);
    }
}

/// the ref of the stk
#[derive(Clone, Copy)]
pub struct OS_STK_REF {
    /// the ref of the stk(top or bottom),because the read of this
    /// field is in the asm code, so we use NonNull to ensure the safety
    /// and use #[allow(dead_code)]
    #[allow(dead_code)]
    pub STK_REF: NonNull<OS_STK>,
    /// the ref of this dynamic stk's src heap
    pub HEAP_REF: NonNull<u8>,
    /// the layout(size) of the stk
    pub layout: Layout,
}
unsafe impl Send for OS_STK_REF {}

/// when the OS_STK_REF is default, we will not alloc a stack
impl Default for OS_STK_REF {
    fn default() -> Self {
        OS_STK_REF {
            STK_REF: NonNull::dangling(),
            HEAP_REF: NonNull::dangling(),
            layout: Layout::from_size_align(0, 1).unwrap(),
        }
    }
}
// /// we impl drop for OS_STK_REF to dealloc the stack(try to be RAII)
// impl Drop for OS_STK_REF {
//     fn drop(&mut self) {
//         unsafe {
//             STACK_ALLOCATOR.dealloc(self.HEAP_REF.as_ptr(), self.layout);
//         }
//     }
// }

impl OS_STK_REF {
    pub fn as_ptr(&self) -> *mut u8 {
        self.HEAP_REF.as_ptr()
    }
}

pub fn stk_from_ptr(stk_ptr: *mut u8, layout: Layout) -> OS_STK_REF {
    OS_STK_REF {
        STK_REF: NonNull::new(unsafe { stk_ptr.offset(layout.size() as isize) as *mut OS_STK }).unwrap(),
        HEAP_REF: NonNull::new(stk_ptr).unwrap(),
        layout,
    }
}
