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

pub const STACK_START: usize = 0x20000000;
pub const STACK_SIZE: usize = 80 * 1024; // 80 KiB
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
    let stk_prt = stk.STK_REF.as_ptr() as *mut u8;
    PROGRAM_STACK.set(stk);
    // then we change the sp to the top of the program stack
    // this depending on the arch so we need extern and implement in the port
    extern "Rust" {
        fn set_program_sp(sp: *mut u8);
    }
    unsafe {
        set_program_sp(stk_prt);
    }
    // we also need to allocate a stack for interrupt
    let layout = Layout::from_size_align(INTERRUPT_STACK_SIZE, 8).unwrap();
    let stk = alloc_stack(layout);
    INTERRUPT_STACK.set(stk);
}
/// alloc a new stack
pub fn alloc_stack(layout: Layout) -> OS_STK_REF {
    unsafe { stk_from_ptr(STACK_ALLOCATOR.alloc(layout), layout) }
}
/// dealloc a stack
pub fn dealloc_stack(stk: &mut OS_STK_REF) {
    if stk.STK_REF == NonNull::dangling() || stk.HEAP_REF == NonNull::dangling() {
        return;
    }
    let stk_ptr = stk.HEAP_REF.as_ptr();
    stk.STK_REF = NonNull::dangling();
    stk.HEAP_REF = NonNull::dangling();
    unsafe {
        STACK_ALLOCATOR.dealloc(stk_ptr, stk.layout);
    }
}

/// the ref of the stk
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
/// we impl drop for OS_STK_REF to dealloc the stack(try to be RAII)
impl Drop for OS_STK_REF {
    fn drop(&mut self) {
        if self.STK_REF == NonNull::dangling() || self.HEAP_REF == NonNull::dangling() {
            return;
        }
        let stk_ptr = self.HEAP_REF.as_ptr();
        self.STK_REF = NonNull::dangling();
        self.HEAP_REF = NonNull::dangling();
        unsafe {
            STACK_ALLOCATOR.dealloc(stk_ptr, self.layout);
        }
    }
}

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
#[cfg(test)]
#[defmt_test::tests]
mod unit_tests {
    use defmt::{assert, println};

    use super::INTERRUPT_STACK;
    use crate::os_core::OSInit;
    use crate::port::os_cpu::set_int_change_2_psp;
    #[init]
    fn init() {
        // this including set allocate stack for psp and msp, and change psp to that value(msp will chage below)
        OSInit();
        // before we step into the loop, we call set_int_change_2_psp(as part of the function of OSStartHighRdy in ucosii)
        // to change the stack pointer to program pointer and use psp
        // let int_stk = INTERRUPT_STACK.exclusive_access();
        // let int_ptr = int_stk.STK_REF.as_ptr() as *mut u8;
        // drop(int_stk);
        // set_int_change_2_psp(int_ptr);
    }
    #[test]
    fn stack_allocator_basic_test() {
        let layout = alloc::alloc::Layout::from_size_align(1024, 8).unwrap();
        let stk = super::alloc_stack(layout);
        assert!(stk.STK_REF.as_ptr() as usize > super::STACK_START);
        assert!((stk.STK_REF.as_ptr() as usize) < (super::STACK_START + super::STACK_SIZE));
        assert!(stk.layout.size() == 1024);
        println!("stk ptr:{:?}", stk.STK_REF.as_ptr());
        assert!(stk.STK_REF.as_ptr() as usize == 0x20000c00);
        // drop(stk);
        let stk = super::alloc_stack(layout);
        println!("stk ptr:{:?}", stk.STK_REF.as_ptr());
        assert!(stk.STK_REF.as_ptr() as usize == 0x20001000);
    }
    #[test]
    fn stack_allocator_med_test() {
        let layout = alloc::alloc::Layout::from_size_align(1024, 8).unwrap();
        let stk1 = super::alloc_stack(layout);
        println!("stk1 ptr:{:?}", stk1.STK_REF.as_ptr());
        assert!(stk1.STK_REF.as_ptr() as usize == 0x20000c00);
        let stk2 = super::alloc_stack(layout);
        println!("stk2 ptr:{:?}", stk2.STK_REF.as_ptr());
        assert!(stk2.STK_REF.as_ptr() as usize == 0x20001000);
        drop(stk1);
        let stk3 = super::alloc_stack(layout);
        println!("stk3 ptr:{:?}", stk3.STK_REF.as_ptr());
        assert!(stk3.STK_REF.as_ptr() as usize == 0x20000c00);
        let stk4 = super::alloc_stack(layout);
        println!("stk4 ptr:{:?}", stk4.STK_REF.as_ptr());
        assert!(stk4.STK_REF.as_ptr() as usize == 0x20001400);
        // let stk5 = super::alloc_stack(layout);
        // println!("stk5 ptr:{:?}", stk5.STK_REF.as_ptr());
        // assert!(stk5.STK_REF.as_ptr() as usize== 0x20001800);
        // 这里drop顺序比较重要，因为方便我们下面的测试接着会用回收的栈
        // 提醒一下这里我故意做了一个2048（2K）的地址对齐，因为底层的linked_list_allocator分配的时候会有保证分配的时候是对齐的
        // drop(stk5);
        drop(stk4);
        drop(stk2);
        drop(stk3);
    }
    #[test]
    fn stack_allocator_hard_test() {
        let layout1 = alloc::alloc::Layout::from_size_align(1024, 8).unwrap();
        let layout2 = alloc::alloc::Layout::from_size_align(2048, 8).unwrap();
        let layout3 = alloc::alloc::Layout::from_size_align(4096, 8).unwrap();
        
        let stk1 = super::alloc_stack(layout1);
        println!("stk1 ptr:{:?}", stk1.STK_REF.as_ptr());
        assert!(stk1.STK_REF.as_ptr() as usize == 0x20000c00);

        let stk2 = super::alloc_stack(layout2);
        println!("stk2 ptr:{:?}", stk2.STK_REF.as_ptr());
        assert!(stk2.STK_REF.as_ptr() as usize == 0x20002000);

        let stk2_1 = super::alloc_stack(layout1);
        println!("stk2_1 ptr:{:?}", stk2_1.STK_REF.as_ptr());
        assert!(stk2_1.STK_REF.as_ptr() as usize == 0x20001000);

        let stk2_2 = super::alloc_stack(layout1);
        println!("stk2_2 ptr:{:?}", stk2_2.STK_REF.as_ptr());
        assert!(stk2_2.STK_REF.as_ptr() as usize == 0x20001400);
        // 注意这个stk2_3的分配很有意思，源于linked_list的对齐
        let stk2_3 = super::alloc_stack(layout1);
        println!("stk2_3 ptr:{:?}", stk2_3.STK_REF.as_ptr());
        assert!(stk2_3.STK_REF.as_ptr() as usize == 0x20001800);

        let stk3 = super::alloc_stack(layout3);
        println!("stk3 ptr:{:?}", stk3.STK_REF.as_ptr());
        assert!(stk3.STK_REF.as_ptr() as usize == 0x20003000);
        drop(stk2);
        let stk4 = super::alloc_stack(layout2);
        println!("stk4 ptr:{:?}", stk4.STK_REF.as_ptr());
        assert!(stk4.STK_REF.as_ptr() as usize == 0x20002000);
        let stk5 = super::alloc_stack(layout2);
        println!("stk5 ptr:{:?}", stk5.STK_REF.as_ptr());
        assert!(stk5.STK_REF.as_ptr() as usize == 0x20003800);
    }
}
