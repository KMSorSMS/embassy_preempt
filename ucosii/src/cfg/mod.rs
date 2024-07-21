use crate::port::*;

/// the const val define the lowest prio
pub const OS_LOWEST_PRIO: INT32U = 63;
/// Size of task variables array (#of INT32U entries) 
pub const OS_TASK_REG_TBL_SIZE: USIZE = 1;
/// Max. number of memory partitions
pub const OS_MAX_MEM_PART: USIZE = 5;
/// Max. number of tasks in your application, MUST be >= 2
pub const OS_MAX_TASKS:USIZE=20;
