/*
********************************************************************************************************************************************
*                                                           task mod
*                                           provide interface about the task of uC/OS-II kernel
********************************************************************************************************************************************
*/

/*
********************************************************************************************************************************************
*                                                           import
********************************************************************************************************************************************
*/

use alloc::string::ToString;
use core::alloc::Layout;
use core::future::Future;
use core::sync::atomic::Ordering::Acquire;

use defmt::info;

use crate::cfg::OS_LOWEST_PRIO;
use crate::executor::{GlobalSyncExecutor, OS_TASK_STORAGE};
use crate::heap::stack_allocator::{dealloc_stack, stk_from_ptr};
use crate::port::{INT8U, OS_STK};
use crate::ucosii::{OSIntNesting, OSRunning, OS_ERR_STATE};
const DEFAULT_REVOKE_STACK_SIZE: usize = 128;

/*
********************************************************************************************************************************************
*                                                           interface
********************************************************************************************************************************************
*/
/// the trait to check whether the return type is unit or never return
pub trait ReturnUnitOrNeverReturn {}

impl ReturnUnitOrNeverReturn for ! {}
impl ReturnUnitOrNeverReturn for () {}
/// Create a task in uC/OS-II kernel. This func is used by C
// _ptos is not used in this func, because stack allocation is done by the stack allocator when scheduling
pub fn SyncOSTaskCreate<F,R>(task: F, p_arg: *mut (), _ptos: *mut OS_STK, prio: INT8U) -> OS_ERR_STATE
where
    // check by liam: why the future is 'static: because the definition of OS_TASK_STORAGE's generic F is 'static
    F: FnOnce(*mut ()) -> R + 'static,
    R: ReturnUnitOrNeverReturn,
{
    // check the priority
    if prio > OS_LOWEST_PRIO as u8 {
        return OS_ERR_STATE::OS_ERR_PRIO_INVALID;
    }

    // warp the normal func to a async func
    let future_func = move || async move { task(p_arg) };
    // info!("the size of future is {}",core::mem::size_of_val(&future_func));
    // if the ptos is not null, we will revoke it as the miniaml stack size(which is 128 B)
    if !_ptos.is_null() {
        let layout = Layout::from_size_align(DEFAULT_REVOKE_STACK_SIZE, 4).unwrap();
        let heap_ptr = unsafe { (_ptos as *mut u8).offset(-(DEFAULT_REVOKE_STACK_SIZE as isize)) };
        // by noah: used to test ffi
        info!("Task Create");
        let mut stk = stk_from_ptr(heap_ptr as *mut u8, layout);
        dealloc_stack(&mut stk);
    }
    return init_task(prio, future_func);
}

/// Create a task in uC/OS-II kernel. This func is used by async Rust
pub fn AsyncOSTaskCreate<F, FutFn>(task: FutFn, p_arg: *mut (), _ptos: *mut OS_STK, prio: INT8U) -> OS_ERR_STATE
where
    // check by liam: why the future is 'static: because the definition of OS_TASK_STORAGE's generic F is 'static
    F: Future + 'static,
    FutFn: FnOnce(*mut ()) -> F + 'static,
{
    let future_func = || task(p_arg);
    // if the ptos is not null, we will revoke it as the miniaml stack size(which is 128 B)
    if !_ptos.is_null() {
        let layout = Layout::from_size_align(DEFAULT_REVOKE_STACK_SIZE, 4).unwrap();
        let heap_ptr = unsafe { (_ptos as *mut u8).offset(-(DEFAULT_REVOKE_STACK_SIZE as isize)) };
        let mut stk = stk_from_ptr(heap_ptr as *mut u8, layout);
        dealloc_stack(&mut stk);
    }
    return init_task(prio, future_func);
}

/// FFI interface
#[no_mangle]
pub extern "C" fn OSTaskCreate(fun_ptr: extern "C" fn(*mut ())->!, p_arg: *mut (), ptos: *mut OS_STK, prio: INT8U) -> OS_ERR_STATE {
    // wrap the task to "F: FnOnce(*mut ()) -> R + 'static"
    // to force the closure to take ownership of `fun_ptr`, use the `move` keyword: `move `
    let task = move|p_arg|{
        fun_ptr(p_arg)
    };
    SyncOSTaskCreate(task, p_arg, ptos, prio)
}

fn init_task<F: Future + 'static>(prio: INT8U, future_func: impl FnOnce() -> F) -> OS_ERR_STATE {
    // Make sure we don't create the task from within an ISR
    if OSIntNesting.load(Acquire) > 0 {
        return OS_ERR_STATE::OS_ERR_TASK_CREATE_ISR;
    }
    // because this func can be call when the OS has started, so need a cs
    if critical_section::with(|_cs| {
        let executor = GlobalSyncExecutor.as_ref().unwrap();
        if executor.prio_exist(prio) {
            return true;
        } else {
            // reserve bit
            executor.reserve_bit(prio);
            return false;
        }
    }) {
        info!("the prio is exist");
        return OS_ERR_STATE::OS_ERR_PRIO_EXIST;
    }

    let err = OS_TASK_STORAGE::init(prio, 0, 0 as *mut (), 0, "".to_string(), future_func);
    if err == OS_ERR_STATE::OS_ERR_NONE {
        // check whether the task is created after the OS has started
        if OSRunning.load(Acquire) {
            // schedule the task, not using poll, we have to make a preemptive schedule
            unsafe{
                GlobalSyncExecutor.as_ref().unwrap().IntCtxSW();
            }
        }
    } else {
        critical_section::with(|_cs| {
            let executor = GlobalSyncExecutor.as_ref().unwrap();
            // clear the reserve bit
            executor.clear_bit(prio);
        })
    }
    return err;
}
