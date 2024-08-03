//! about the cpu

use core::arch::asm;

use defmt::info;

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
pub extern "Rust" fn set_int_change_2_psp(int_ptr: *mut u8){
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
