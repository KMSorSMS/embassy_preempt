//! the async scheduler(executor) of rust uC
/*
****************************************************************************************************************************************
*                                                      the scheduler of uC/OS-II
*                                                        code by liam and noah  
****************************************************************************************************************************************
*/

use core::future::Future;
use core::ptr::NonNull;

use alloc::string::String;

use crate::port::*;
use crate::ucosii::*;
use crate::cfg::*;
use crate::stk_allocator::OS_STK_REF;
use crate::util::SyncUnsafeCell;
use crate::util::UninitCell;

/*
****************************************************************************************************************************************
*                                                             type define  
****************************************************************************************************************************************
*/

/// the TCB of the task. It contains the task's info
#[allow(unused)]
// we put it in executor crate to use "pub(crate)" to make it can be used in the other mod in order to reduce coupling
pub(crate) struct OS_TCB{
    // it maybe None
    OSTCBStkPtr:Option<OS_STK_REF>, /* Pointer to current top of stack                         */
    // Task specific extension. If the OS_TASK_CREATE_EXT_EN feature is not active, it will be None
    OSTCBExtInfo:Option<OS_TCB_EXT>, 

    OSTCBNext:Option<OS_TCB_REF>, /* Pointer to next     TCB in the TCB list                 */
    OSTCBPrev:Option<OS_TCB_REF>, /* Pointer to previous TCB in the TCB list                 */

    // the poll fn that will be called by the executor. In the func, a waker will be create.
    OS_POLL_FN:SyncUnsafeCell<Option<unsafe fn(OS_TCB_REF)>>,
    
    #[cfg(feature="OS_EVENT_EN")]
    OSTCBEventPtr:Option<OS_EVENT_REF>, /* Pointer to event control block                */
    
    #[cfg(any(all(feature = "OS_Q_EN", feature = "OS_MAX_QS"), feature = "OS_MBOX_EN"))]
    OSTCBMsg:PTR, /* Message received from OSMboxPost() or OSQPost()         */
    
    OSTCBDly:INT32U,     /* Nbr ticks to delay task or, timeout waiting for event   */
    OSTCBStat:INT8U,     /* Task      status                                        */
    OSTCBStatPend:INT8U, /* Task PEND status                                        */
    OSTCBPrio:INT8U,     /* Task priority (0 == highest)                            */
    
    OSTCBX:INT8U,      /* Bit position in group  corresponding to task priority   */
    OSTCBY:INT8U,      /* Index into ready table corresponding to task priority   */
    OSTCBBitX:OS_PRIO, /* Bit mask to access bit position in ready table          */
    OSTCBBitY:OS_PRIO, /* Bit mask to access bit position in ready group          */
    
    #[cfg(feature="OS_TASK_DEL_EN")]
    OSTCBDelReq:INT8U,  /* Indicates whether a task needs to delete itself         */
    
    #[cfg(feature="OS_TASK_PROFILE_EN")]
    OSTCBCtxSwCtr:INT32U,    /* Number of time the task was switched in                 */
    OSTCBCyclesTot:INT32U,   /* Total number of clock cycles the task has been running  */
    OSTCBCyclesStart:INT32U, /* Snapshot of cycle counter at start of task resumption   */
    OSTCBStkBase:OS_STK,    /* Pointer to the beginning of the task stack              */
    OSTCBStkUsed:INT32U,     /* Number of bytes used from the stack                     */
    
    #[cfg(feature="OS_TASK_REG_TBL_SIZE")]
    OSTCBRegTbl:[INT32U;OS_TASK_REG_TBL_SIZE],
    
    #[cfg(feature="OS_TASK_NAME_EN")]
    OSTCBTaskName: String,
    // #[cfg(feature="OS_TASK_CREATE_EXT_EN")]
    // OS_TLS OSTCBTLSTbl[OS_TLS_TBL_SIZE];
    
    // OS_EVENT **OSTCBEventMultiPtr; /* Pointer to multiple  event control blocks               */
    // OS_EVENT *OSTCBEventMultiRdy;  /* Pointer to the first event control block readied        */
    // OS_FLAG_NODE *OSTCBFlagNode; /* Pointer to event flag node                              */

    // OS_FLAGS OSTCBFlagsRdy; /* Event flags that made task ready to run                 */
}

#[cfg(feature="OS_TASK_CREATE_EXT_EN")]
#[allow(unused)]
pub(crate) struct OS_TCB_EXT{
    OSTCBExtPtr:PTR,      /* Pointer to user definable data for TCB extension        */
    OSTCBStkBottom:Option<OS_STK_REF>, /* Pointer to bottom of stack                              */
    OSTCBStkSize:INT32U,    /* Size of task stack (in number of stack elements)        */
    OSTCBOpt:INT16U,        /* Task options as passed by OSTaskCreateExt()             */
    OSTCBId:INT16U,         /* Task ID (0..65535)                                      */
}

/// the storage of the task. It contains the task's TCB and the future
#[allow(unused)]
pub struct OS_TASK_STORAGE<F: Future + 'static>{
    task_tcb:OS_TCB,
    // this part is invisible to other crate
    future:UninitCell<F>,
}

/// the ref of the TCB. In other crate only it can be used to access the TCB
#[derive(Clone, Copy)]
#[allow(unused)]
pub struct OS_TCB_REF{
    ptr:NonNull<OS_TCB>,
}

/*
****************************************************************************************************************************************
*                                                             implement of structure  
****************************************************************************************************************************************
*/

impl <F: Future + 'static>OS_TASK_STORAGE<F>{
    /// create a new OS_TASK_STORAGE
    // Take a lazy approach, which means the TCB will be init when call the init func of TCB
    // this func will be used to init the global array
    pub const fn new()->Self{
        Self{
            task_tcb:OS_TCB{
                OSTCBStkPtr:None,
                OSTCBExtInfo:None,
                OSTCBNext:None,
                OSTCBPrev:None,
                OS_POLL_FN:SyncUnsafeCell::new(None),
                #[cfg(feature="OS_EVENT_EN")]
                OSTCBEventPtr:None,
                #[cfg(any(all(feature = "OS_Q_EN", feature = "OS_MAX_QS"), feature = "OS_MBOX_EN"))]
                OSTCBMsg:0 as PTR,
                OSTCBDly:0,
                OSTCBStat:0,
                OSTCBStatPend:0,
                OSTCBPrio:0,
                OSTCBX:0,
                OSTCBY:0,
                OSTCBBitX:0,
                OSTCBBitY:0,
                #[cfg(feature="OS_TASK_DEL_EN")]
                OSTCBDelReq:0,
                #[cfg(feature="OS_TASK_PROFILE_EN")]
                OSTCBCtxSwCtr:0,
                OSTCBCyclesTot:0,
                OSTCBCyclesStart:0,
                OSTCBStkBase:0,
                OSTCBStkUsed:0,
                #[cfg(feature="OS_TASK_REG_TBL_SIZE")]
                OSTCBRegTbl:[0;OS_TASK_REG_TBL_SIZE],
                #[cfg(feature="OS_TASK_NAME_EN")]
                OSTCBTaskName:String::new(),
            },
            future:UninitCell::uninit(),
        }
    }

    /// init the storage of the task, just like the spawn in Embassy
    //  this func will be called by OS_TASK_CTREATE
    pub fn init(&'static self, _future:impl FnOnce() -> F){
        
    }

    /// the poll fun called by the executor
    pub fn poll(){

    }
}


impl OS_TCB_REF{
    
}