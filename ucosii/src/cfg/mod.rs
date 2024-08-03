use crate::port::*;

/// the const val define the lowest prio
pub const OS_LOWEST_PRIO: USIZE = 63;
/// Size of task variables array (#of INT32U entries) 
pub const OS_TASK_REG_TBL_SIZE: USIZE = 1;
/// Max. number of memory partitions
pub const OS_MAX_MEM_PART: USIZE = 5;
/// Max. number of tasks in your application, MUST be >= 2
pub const OS_MAX_TASKS:USIZE=20;
/// This const val is used to config the size of ARENA. 
/// You can set it refer to the number of tasks in your application(OS_MAX_TASKS) and the number of system tasks(OS_N_SYS_TASKS).
pub const OS_ARENA_SIZE: USIZE = 1024;
/// frequency of the Systick(run on Timer)
pub const OS_TICK_FREQ:USIZE=5000;