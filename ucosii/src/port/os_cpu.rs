//! about the cpu

use core::arch::asm;

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
pub extern "Rust" fn run_idle() {
    // undate the counter of the system
    // OSIdleCtr.fetch_add(1, Ordering::Relaxed);
    unsafe {
        asm!("wfe");
    }
}
