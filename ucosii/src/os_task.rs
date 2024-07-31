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

use core::{future::{self, Future}, ptr::null_mut};

use alloc::string::ToString;

use crate::{cfg::OS_LOWEST_PRIO, executor::OS_TASK_STORAGE, heap::stack_allocator::OS_STK_REF, port::{INT8U, OS_STK}, ucosii::OS_ERR_STATE};

/*
********************************************************************************************************************************************
*                                                           interface
********************************************************************************************************************************************
*/


/// Create a task in uC/OS-II kernel. This func is used by C
// _ptos is not used in this func, because stack allocation is done by the stack allocator when scheduling
pub fn OSTaskCreate<F>(task: F, p_arg:*mut (), _ptos: *mut OS_STK, prio: INT8U) -> OS_ERR_STATE
where
// check by liam: why the future is 'static: because the definition of OS_TASK_STORAGE's generic F is 'static
    F: FnOnce(*mut ()) + 'static,
{
    // check the priority
    if prio >= OS_LOWEST_PRIO as u8 {
        return OS_ERR_STATE::OS_ERR_PRIO_INVALID;
    }

    // warp the normal func to a async func
    let future_func = move ||{
        async move{
            task(p_arg)
        }
    };
    //
    OS_TASK_STORAGE::init(prio, 0, 0 as *mut (), 0, "".to_string(), future_func);
    return OS_ERR_STATE::OS_ERR_NONE;
}

/// Create a task in uC/OS-II kernel. This func is used by async Rust
pub fn RustOSTaskCreate<F,FutFn>(task: FutFn, p_arg:*mut (), _ptos: *mut OS_STK, prio: INT8U)->OS_ERR_STATE
where
// check by liam: why the future is 'static: because the definition of OS_TASK_STORAGE's generic F is 'static
    F: Future + 'static,
    FutFn: FnOnce(*mut ()) -> F + 'static,
{
    // warp the normal func to a async func
    let future_func = move ||{
        async move{
            // only this part is different to OSTaskCreate
            task(p_arg).await
        }
    };
    
    OS_TASK_STORAGE::init(prio, 0, 0 as *mut (), 0, "".to_string(), future_func);
    return OS_ERR_STATE::OS_ERR_NONE;
}