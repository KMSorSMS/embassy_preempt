//! about the cpu

use core::arch::asm;
use core::ptr::NonNull;

use cortex_m_rt::exception;
#[cfg(feature = "defmt")]
use defmt::info;
use defmt::trace;

use super::OS_STK;
use crate::executor::GlobalSyncExecutor;
use crate::heap::stack_allocator::{INTERRUPT_STACK, PROGRAM_STACK};

// use crate::ucosii::OSIdleCtr;
// use core::sync::atomic::Ordering::Relaxed;

// use crate::heap::init_heap;

/// finish the init part of the CPU/MCU
pub fn OSInitHookBegin() {
    // if we need heap, we can init it here
    // init_heap();
}

const NVIC_INT_CTRL: u32 = 0xE000ED04;
const NVIC_PENDSVSET: u32 = 0x10000000;
#[no_mangle]
#[inline]
/// the function to start the first task
pub extern "Rust" fn restore_thread_task() {
    #[cfg(feature = "defmt")]
    trace!("restore_thread_task");
    unsafe {
        asm!(
            "STR     R1, [R0]",
            in("r0") NVIC_INT_CTRL,
            in("r1") NVIC_PENDSVSET,
        )
    }
}

// the pendsv handler used to switch the task
#[exception]
fn PendSV() {
    // first close the interrupt
    unsafe {
        asm!(
            "CPSID I",
            "MRS     R0, PSP",
            // save the context
            "STMFD   R0!, {{R4-R11, R14}}",
            // fix: we need to write back to the PSP
            "MSR     PSP, R0",
            // "CPSIE   I",
            options(nostack, preserves_flags)
        );
    }
    #[cfg(feature = "defmt")]
    trace!("PendSV");
    // then switch the task
    #[cfg(feature = "defmt")]
    info!(
        "in pendsv the highrdy task's prio is : {:?}",
        GlobalSyncExecutor.as_ref().unwrap().OSPrioHighRdy.get_unmut()
    );
    if GlobalSyncExecutor.as_ref().unwrap().OSPrioHighRdy.get_unmut()
        == GlobalSyncExecutor.as_ref().unwrap().OSPrioCur.get_unmut()
    {
        #[cfg(feature = "defmt")]
        info!("the highrdy is the same as the current, no need to switch");
        // we will reset the msp to the original
        let msp_stk = INTERRUPT_STACK.get().STK_REF.as_ptr();
        unsafe {
            asm!(
                // "CPSID I",
                "MRS    R0, PSP",
                "LDMFD   R0!, {{R4-R11, R14}}",
                "MSR     PSP, R0",
                // reset the msp
                "MSR     MSP, R1",
                "CPSIE   I",
                "BX      LR",
                in("r1") msp_stk,
                options(nostack, preserves_flags),
            )
        }
    }
    let stk_ptr: crate::heap::stack_allocator::OS_STK_REF =
        GlobalSyncExecutor.as_ref().unwrap().OSTCBHighRdy.get_mut().take_stk();
    let program_stk_ptr = stk_ptr.STK_REF.as_ptr();
    // the swap will return the ownership of PROGRAM_STACK's original value and set the new value(check it when debuging!!!)
    let mut old_stk = PROGRAM_STACK.swap(stk_ptr);
    // by noah: *TEST*
    // let TCB: &OS_TCB;
    if !*GlobalSyncExecutor
        .as_ref()
        .unwrap()
        .OSTCBCur
        .get_unmut()
        .is_in_thread_poll
        .get_unmut()
    {
        // this situation is in interrupt poll
        #[cfg(feature = "defmt")]
        info!("need to save the context");
        // we need to give the current task the old_stk to store the context
        // first we will store the remaining context to the old_stk
        let old_stk_ptr: *mut usize;
        unsafe {
            asm!(
                "MRS     R0, PSP",
                out("r0") old_stk_ptr,
                options(nostack, preserves_flags),
            )
        }
        // then as we have stored the context, we need to update the old_stk's top
        old_stk.STK_REF = NonNull::new(old_stk_ptr as *mut OS_STK).unwrap();
        #[cfg(feature = "defmt")]
        info!("in pendsv, the old stk ptr is {:?}", old_stk_ptr);
        // GlobalSyncExecutor.as_ref().unwrap().OSTCBCur.get_mut().set_stk(old_stk)
        let task_cur = GlobalSyncExecutor.as_ref().unwrap().OSTCBCur.get_mut();
        task_cur.set_stk(old_stk);
        // get the TCB
        // unsafe {
        //     TCB = task_cur.ptr.unwrap().as_ref();
        // }
        // by noah: judge whether the task stk is none
        if task_cur.is_stk_none() {
            #[cfg(feature = "defmt")]
            info!("the task stk is none");
        }
    } else {
        // the situation is in poll
        drop(old_stk);
    }
    // set the current task to be the highrdy
    unsafe {
        critical_section::with(|cs|{
            GlobalSyncExecutor.as_ref().unwrap().set_cur_highrdy(cs);
        });
        // set the current task's is_in_thread_poll to true
        GlobalSyncExecutor
            .as_ref()
            .unwrap()
            .OSTCBCur
            .get_mut()
            .is_in_thread_poll
            .set(true);
    }
    #[cfg(feature = "defmt")]
    info!("trying to restore, the new stack pointer is {:?}", program_stk_ptr);
    // we will reset the msp to the original
    let msp_stk = INTERRUPT_STACK.get().STK_REF.as_ptr();
    unsafe {
        asm!(
            // "CPSID I",
            "LDMFD   R0!, {{R4-R11, R14}}",
            "MSR     PSP, R0",
            // reset the msp
            "MSR     MSP, R1",
            "CPSIE   I",
            "BX      LR",
            in("r0") program_stk_ptr,
            in("r1") msp_stk,
            options(nostack, preserves_flags),
        )
    }
}

#[no_mangle]
/// the function when there is no task to run
pub extern "Rust" fn run_idle() {
    #[cfg(feature = "defmt")]
    trace!("run_idle");
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
#[repr(C, align(4))]
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
pub extern "Rust" fn OSTaskStkInit(stk_ref: NonNull<OS_STK>) -> NonNull<OS_STK> {
    #[cfg(feature = "defmt")]
    trace!("OSTaskStkInit");
    let executor_function_ptr: fn() = || unsafe {
        #[cfg(feature = "defmt")]
        info!("entering the executor function");
        GlobalSyncExecutor.as_ref().unwrap().poll();
    };
    let executor_function_ptr = executor_function_ptr as *const () as usize;
    let ptos = stk_ref.as_ptr() as *mut usize;
    // do align with 8 and move the stack pointer down an align size
    let mut ptos = ((unsafe { ptos.offset(1) } as usize) & 0xFFFFFFF8) as *mut usize;
    ptos = unsafe { ptos.offset(-(CONTEXT_STACK_SIZE as isize + 1) as isize) };
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
    #[cfg(feature = "defmt")]
    trace!("set_program_sp");
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
    #[cfg(feature = "defmt")]
    trace!("set_int_change_2_psp");
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
