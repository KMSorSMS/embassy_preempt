//! about the cpu

use core::{arch::asm, ptr::NonNull};

use crate::executor::GlobalSyncExecutor;

use super::OS_STK;

// use crate::ucosii::OSIdleCtr;
// use core::sync::atomic::Ordering::Relaxed;

// use crate::heap::init_heap;

/// finish the init part of the CPU/MCU
pub fn OSInitHookBegin() {
    // if we need heap, we can init it here
    // init_heap();
}

#[no_mangle]
/// the function to start the first task
pub extern "Rust" fn restore_arch_stk_user(stk: *mut usize) {
    unsafe {
        asm!(
            // first restore the xpsr(from r0)
            "LDR r0, [r0, #64]",
            "MSR xpsr, r0",
            // then we restore the context
            "LDMFD  r0!, {{r4-r11, r14}}",
            // then set the psp as stk
            "MSR psp, r0",
            // then we need to restore the interrupt context
            "LDMFD  sp!, {{r0-r3, r12, lr}}",
            // then we need to jump to the task(resotre pc),8 is important, we need to skip xpsr
            "LDR  pc,[sp], #8",
            in("r0") stk,
            options(nostack, preserves_flags),
        )
    }
}

#[no_mangle]
/// the function when there is no task to run
pub extern "Rust" fn run_idle() {
    // undate the counter of the system
    // OSIdleCtr.fetch_add(1, Ordering::Relaxed);
    unsafe {
        asm!("wfe");
    }
}

// #[no_mangle]
// #[inline]
// /// the function to return from interrupt(cortex-m)
// pub extern "Rust" fn OSIntExit(){
//     unsafe {
//         asm!(
            
//         )
//     }
// }

/// the context structure store in stack
#[repr(C, align(8))]
struct UcStk {
    // below are the remaining part of the task's context
    r4: u32,
    r5: u32,
    r6: u32,
    r7: u32,
    r8: u32,
    r9: u32,
    r10: u32,
    r11: u32,
    r14: u32,
    // below are stored when the interrupt occurs
    r0: u32,
    r1: u32,
    r2: u32,
    r3: u32,
    r12: u32,
    lr: u32, 
    pc: u32,
    xpsr: u32,
}
const CONTEXT_STACK_SIZE: usize = 16;

#[no_mangle]
#[inline]
/// the function to mock/init the stack of the task
/// set the pc to the executor's poll function
pub extern "Rust" fn OSTaskStkInit(stk_ref:NonNull<OS_STK>) -> NonNull<OS_STK> {
    let executor_function_ptr: fn() = || unsafe { GlobalSyncExecutor.as_ref().unwrap().poll() };
    let executor_function_ptr = executor_function_ptr as *const () as usize;
    let ptos = stk_ref.as_ptr() as *mut usize;
    let ptos = ((unsafe {ptos.offset(1) } as usize) & 0xFFFFFFF8) as *mut usize;
    let ptos = unsafe { ptos.offset(-(CONTEXT_STACK_SIZE as isize) as isize) };
    let psp = ptos as *mut UcStk;
    // initialize the stack
    unsafe {
        (*psp).r0 = 0;
        (*psp).r1 = 0x01010101;
        (*psp).r2 = 0x02020202;
        (*psp).r3 = 0x03030303;
        (*psp).r4 = 0x04040404;
        (*psp).r5 = 0x05050505;
        (*psp).r6 = 0x06060606;
        (*psp).r7 = 0x07070707;
        (*psp).r8 = 0x08080808;
        (*psp).r9 = 0x09090909;
        (*psp).r10 = 0x10101010;
        (*psp).r11 = 0x11111111;
        (*psp).r12 = 0x12121212;
        (*psp).r14 = 0xFFFFFFFD;
        (*psp).lr = 0;
        (*psp).pc = executor_function_ptr as u32;
        (*psp).xpsr = 0x01000000;
    }
    // return the new stack pointer
    NonNull::new(ptos as *mut OS_STK).unwrap()
}

#[no_mangle]
#[inline]
/// the function to set the program stack
pub extern "Rust" fn set_program_sp(sp: *mut u8) {
    unsafe {
        asm!(
            "MSR psp, r0",
            in("r0") sp,
            options(nostack, preserves_flags),
        )
    }
}
#[no_mangle]
#[inline]
/// the function to set the interrupt stack and change the control register to use the psp
pub extern "Rust" fn set_int_change_2_psp(int_ptr: *mut u8) {
    unsafe {
        asm!(
            // fisrt change the MSP
           "MSR msp, r1",
            // then change the control register to use the psp
            "MRS r0, control",
            "ORR r0, r0, #2",
            "MSR control, r0",
            // make sure the function will be inlined as we don't use lr to return
            // // then we need to return to the caller, this time we explicitly use the lr
            // "BX lr",
            in("r1") int_ptr,
            options(nostack, preserves_flags),
        )
    }
}
