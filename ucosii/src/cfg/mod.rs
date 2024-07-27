use crate::port::*;

/// the const val define the lowest prio
pub const OS_LOWEST_PRIO: INT32U = 63;
/// Size of task variables array (#of INT32U entries) 
pub const OS_TASK_REG_TBL_SIZE: USIZE = 1;
/// Max. number of memory partitions
pub const OS_MAX_MEM_PART: USIZE = 5;
/// Max. number of tasks in your application, MUST be >= 2
pub const OS_MAX_TASKS:USIZE=20;
/// This const val is used to config the size of ARENA. 
/// You can set it refer to the number of tasks in your application(OS_MAX_TASKS) and the number of system tasks(OS_N_SYS_TASKS).
pub const OS_ARENA_SIZE: USIZE = 4096;
// /// by noah: because we need to provide the byte number of the Arena, but the size of OS_TASK_STORAGE can not be confirmed at compiling time.
// /// So I just define the size of OS_TASK_STORAGE. Maybe there will be a better way to solve this problem...
// pub const OS_TASK_STORAGE_SIZE: USIZE = core::mem::size_of::<OS_TCB>()+FUTURE_SIZE;
// pub const FUTURE_SIZE:USIZE=;
// by noah:maybe we can postpone the initialization of OSTCBTbl to OSStart because at that time the size of future is known.
// Aha, I think I can alloc the future in heap to make OS_TASK_STORAGE Sized.(I will try this method)