/*
*********************************************************************************************************
*                                              uC/OS-II
*                                        The Real-Time Kernel
*
*                    Copyright 1992-2021 Silicon Laboratories Inc. www.silabs.com
*
*                                 SPDX-License-Identifier: APACHE-2.0
*
*               This software is subject to an open source license and is distributed by
*                Silicon Laboratories Inc. pursuant to the terms of the Apache License,
*                    Version 2.0 available at www.apache.org/licenses/LICENSE-2.0.
*                                    rewrite by liam and noah
*
*********************************************************************************************************
*/

/*
*********************************************************************************************************
* Filename : ucos_ii.rs
* Version  : V0.0.1
*********************************************************************************************************
*/

/*
*********************************************************************************************************
*                                   The mods that ucosii.rs depends on 
*********************************************************************************************************
*/

use crate::port::*;
use core::ptr::NonNull;
use core::env;
#[macro_use]
extern crate lazy_static;

/*
*********************************************************************************************************
*                                            MISCELLANEOUS
*********************************************************************************************************
*/

const OS_PRIO_SELF:INT32U = 0xFF;           /* Indicate SELF priority                      */
const OS_PRIO_MUTEX_CEIL_DIS:INT32U = 0xFF; /* Disable mutex priority ceiling promotion    */

#[cfg(feature = "OS_TASK_STAT_EN")]
const OS_N_SYS_TASKS:INT32U = 2; /* Number of system tasks                      */
#[cfg(not(feature = "OS_TASK_STAT_EN"))]
const OS_N_SYS_TASKS:INT32U = 1;

// lazy_static! {
//     static ref OS_LOWEST_PRIO: INT32U = env!("OS_LOWEST_PRIO").parse().expect("Failed to parse OS_LOWEST_PRIO");
//     static ref OS_TASK_STAT_PRIO: INT32U = *OS_LOWEST_PRIO - 1;
//     static ref OS_TASK_IDLE_PRIO: INT32U = *OS_LOWEST_PRIO;
// }

// by noah：maybe because the lazy_static, the const val can be calculate when it is used for the first time
// maybe use a static val is a good choice.
const OS_TASK_STAT_PRIO:INT32U = env!("OS_LOWEST_PRIO").parse::<INT32U>().unwrap() - 1; /* Statistic task priority                     */
const OS_TASK_IDLE_PRIO:INT32U = env!("OS_LOWEST_PRIO").parse::<INT32U>().unwrap();      /* IDLE      task priority                     */

#[cfg(feature="OS_PRIO_LESS_THAN_64")]
const OS_EVENT_TBL_SIZE:INT32U = env!("OS_LOWEST_PRIO").parse::<INT32U>().unwrap() / 8 + 1; /* Size of event table                         */
#[cfg(feature="OS_PRIO_LESS_THAN_256")]
const OS_EVENT_TBL_SIZE:INT32U = env!("OS_LOWEST_PRIO").parse::<INT32U>().unwrap() / 16 + 1; /* Size of event table                         */

const OS_RDY_TBL_SIZE:INT32U = OS_EVENT_TBL_SIZE;   /* Size of ready table                         */

const OS_TASK_IDLE_ID:INT32U = 65535; /* ID numbers for Idle, Stat and Timer tasks   */
const OS_TASK_STAT_ID:INT32U = 65534;
const OS_TASK_TMR_ID:INT32U = 65533;

/*
*********************************************************************************************************
*                                       uC/OS-II VERSION NUMBER
*********************************************************************************************************
*/



#[allow(unused)]
const OS_VERSION:INT32U = 29300; /* Version of uC/OS-II (Vx.yy mult. by 10000)  */

/*
*********************************************************************************************************
*                             TASK STATUS (Bit definition for OSTCBStat)
*********************************************************************************************************
*/

#[allow(unused)]
const OS_STAT_RDY:INT32U = 0x00;     /* Ready to run                                            */
#[allow(unused)]
const OS_STAT_SEM:INT32U = 0x01;     /* Pending on semaphore                                    */
#[allow(unused)]
const OS_STAT_MBOX:INT32U = 0x02;    /* Pending on mailbox                                      */
#[allow(unused)]
const OS_STAT_Q:INT32U = 0x04;       /* Pending on queue                                        */
#[allow(unused)]
const OS_STAT_SUSPEND:INT32U = 0x08; /* Task is suspended                                       */
#[allow(unused)]
const OS_STAT_MUTEX:INT32U = 0x10;   /* Pending on mutual exclusion semaphore                   */
#[allow(unused)]
const OS_STAT_FLAG:INT32U = 0x20;    /* Pending on event flag group                             */
#[allow(unused)]
const OS_STAT_MULTI:INT32U = 0x80;   /* Pending on multiple events                              */
#[allow(unused)]
const OS_STAT_PEND_ANY:INT32U = (OS_STAT_SEM | OS_STAT_MBOX | OS_STAT_Q | OS_STAT_MUTEX | OS_STAT_FLAG);

/*
*********************************************************************************************************
*                          TASK PEND STATUS (Status codes for OSTCBStatPend)
*********************************************************************************************************
*/

#[allow(unused)]
const OS_STAT_PEND_OK:INT32U = 0;    /* Pending status OK, not pending, or pending complete     */
#[allow(unused)]
const OS_STAT_PEND_TO:INT32U = 1;    /* Pending timed out                                       */
#[allow(unused)]
const OS_STAT_PEND_ABORT:INT32U = 2; /* Pending aborted                                         */

/*
*********************************************************************************************************
*                                           OS_EVENT types
*********************************************************************************************************
*/

#[allow(unused)]
const OS_EVENT_TYPE_UNUSED:INT32U = 0;
#[allow(unused)]
const OS_EVENT_TYPE_MBOX:INT32U = 1;
#[allow(unused)]
const OS_EVENT_TYPE_Q:INT32U = 2;
#[allow(unused)]
const OS_EVENT_TYPE_SEM:INT32U = 3;
#[allow(unused)]
const OS_EVENT_TYPE_MUTEX:INT32U = 4;
#[allow(unused)]
const OS_EVENT_TYPE_FLAG:INT32U = 5;
#[allow(unused)]
const OS_TMR_TYPE:INT32U = 100; /* Used to identify Timers ...                             */
                         /* ... (Must be different value than OS_EVENT_TYPE_xxx)    */

/*
*********************************************************************************************************
*                                             EVENT FLAGS
*********************************************************************************************************
*/

#[allow(unused)]
const OS_FLAG_WAIT_CLR_ALL:INT32U = 0; /* Wait for ALL    the bits specified to be CLR (i.e. 0)   */
#[allow(unused)]
const OS_FLAG_WAIT_CLR_AND:INT32U = 0;

#[allow(unused)]
const OS_FLAG_WAIT_CLR_ANY:INT32U = 1; /* Wait for ANY of the bits specified to be CLR (i.e. 0)   */
#[allow(unused)]
const OS_FLAG_WAIT_CLR_OR:INT32U = 1;

#[allow(unused)]
const OS_FLAG_WAIT_SET_ALL:INT32U = 2; /* Wait for ALL    the bits specified to be SET (i.e. 1)   */
#[allow(unused)]
const OS_FLAG_WAIT_SET_AND:INT32U = 2;

#[allow(unused)]
const OS_FLAG_WAIT_SET_ANY:INT32U = 3; /* Wait for ANY of the bits specified to be SET (i.e. 1)   */
#[allow(unused)]
const OS_FLAG_WAIT_SET_OR:INT32U = 3;

#[allow(unused)]
const OS_FLAG_CONSUME:INT32U = 0x80; /* Consume the flags if condition(s) satisfied             */

#[allow(unused)]
const OS_FLAG_CLR:INT32U = 0;
#[allow(unused)]
const OS_FLAG_SET:INT32U = 1;

/*
*********************************************************************************************************
*                                     Values for OSTickStepState
*
* Note(s): This feature is used by uC/OS-View.
*********************************************************************************************************
*/

/*
*********************************************************************************************************
*      Possible values for 'opt' argument of OSSemDel(), OSMboxDel(), OSQDel() and OSMutexDel()
*********************************************************************************************************
*/

#[allow(unused)]
const OS_DEL_NO_PEND:INT32U = 0;
#[allow(unused)]
const OS_DEL_ALWAYS:INT32U = 1;

/*
*********************************************************************************************************
*                                        OS???Pend() OPTIONS
*
* These #defines are used to establish the options for OS???PendAbort().
*********************************************************************************************************
*/

#[allow(unused)]
const OS_PEND_OPT_NONE:INT32U = 0;      /* NO option selected                                      */
#[allow(unused)]
const OS_PEND_OPT_BROADCAST:INT32U = 1; /* Broadcast action to ALL tasks waiting                   */

/*
*********************************************************************************************************
*                                     OS???PostOpt() OPTIONS
*
* These #defines are used to establish the options for OSMboxPostOpt() and OSQPostOpt().
*********************************************************************************************************
*/

#[allow(unused)]
const OS_POST_OPT_NONE:INT32U = 0x00;      /* NO option selected                                      */
#[allow(unused)]
const OS_POST_OPT_BROADCAST:INT32U = 0x01; /* Broadcast message to ALL tasks waiting                  */
#[allow(unused)]
const OS_POST_OPT_FRONT:INT32U = 0x02;     /* Post to highest priority task waiting                   */
#[allow(unused)]
const OS_POST_OPT_NO_SCHED:INT32U = 0x04;  /* Do not call the scheduler if this option is selected    */

/*
*********************************************************************************************************
*                                TASK OPTIONS (see OSTaskCreateExt())
*********************************************************************************************************
*/

const OS_TASK_OPT_NONE:INT32U = 0x0000;    /* NO option selected                                      */
const OS_TASK_OPT_STK_CHK:INT32U = 0x0001; /* Enable stack checking for the task                      */
const OS_TASK_OPT_STK_CLR:INT32U = 0x0002; /* Clear the stack when the task is create                 */
const OS_TASK_OPT_SAVE_FP:INT32U = 0x0004; /* Save the contents of any floating-point registers       */
const OS_TASK_OPT_NO_TLS:INT32U = 0x0008;  /* Specify that task doesn't needs TLS                     */

/*
*********************************************************************************************************
*                          TIMER OPTIONS (see OSTmrStart() and OSTmrStop())
*********************************************************************************************************
*/

const OS_TMR_OPT_NONE:INT32U = 0; /* No option selected                                      */

const OS_TMR_OPT_ONE_SHOT:INT32U = 1; /* Timer will not automatically restart when it expires    */
const OS_TMR_OPT_PERIODIC:INT32U = 2; /* Timer will     automatically restart when it expires    */

const OS_TMR_OPT_CALLBACK:INT32U = 3;     /* OSTmrStop() option to call 'callback' w/ timer arg.     */
const OS_TMR_OPT_CALLBACK_ARG:INT32U = 4; /* OSTmrStop() option to call 'callback' w/ new   arg.     */

/*
*********************************************************************************************************
*                                            TIMER STATES
*********************************************************************************************************
*/

const OS_TMR_STATE_UNUSED:INT32U = 0;
const OS_TMR_STATE_STOPPED:INT32U = 1;
const OS_TMR_STATE_COMPLETED:INT32U = 2;
const OS_TMR_STATE_RUNNING:INT32U = 3;

/*
*********************************************************************************************************
*                                             ERROR CODES
*********************************************************************************************************
*/

#[derive(PartialEq)]
#[repr(align(8))]
#[allow(non_camel_case_types)]
/// uC/OS-II error codes
pub enum OsErrState {
    /// No error
    OS_ERR_NONE,
    /// The event type is invalid
    OS_ERR_EVENT_TYPE,
    /// The task is pending due to an interrupt service routine (ISR).
    OS_ERR_PEND_ISR,
    /// The pointer to post is NULL
    OS_ERR_POST_NULL_PTR,
    /// The event pointer is NULL
    OS_ERR_PEVENT_NULL,
    /// The post operation was called from an ISR
    OS_ERR_POST_ISR,
    /// The query operation was called from an ISR
    OS_ERR_QUERY_ISR,
    /// The option is invalid
    OS_ERR_INVALID_OPT,
    /// The ID is invalid
    OS_ERR_ID_INVALID,
    /// The pointer to data is NULL
    OS_ERR_PDATA_NULL,

    /// The operation timed out
    OS_ERR_TIMEOUT,
    /// The event name is too long
    OS_ERR_EVENT_NAME_TOO_LONG,
    /// The pointer to name is NULL
    OS_ERR_PNAME_NULL,
    /// The pend operation is locked
    OS_ERR_PEND_LOCKED,
    /// The pend operation was aborted
    OS_ERR_PEND_ABORT,
    /// The delete operation was called from an ISR
    OS_ERR_DEL_ISR,
    /// The create operation was called from an ISR
    OS_ERR_CREATE_ISR,
    /// The get name operation was called from an ISR
    OS_ERR_NAME_GET_ISR,
    /// The set name operation was called from an ISR
    OS_ERR_NAME_SET_ISR,
    /// The create operation is illegal at runtime
    OS_ERR_ILLEGAL_CREATE_RUN_TIME,

    /// The mailbox is full
    OS_ERR_MBOX_FULL,
    /// The delete operation is illegal at runtime
    OS_ERR_ILLEGAL_DEL_RUN_TIME,

    /// The queue is full
    OS_ERR_Q_FULL,
    /// The queue is empty
    OS_ERR_Q_EMPTY,

    /// The priority already exists
    OS_ERR_PRIO_EXIST,
    /// The priority is invalid
    OS_ERR_PRIO,
    /// The priority is invalid
    OS_ERR_PRIO_INVALID,

    /// The scheduler is locked
    OS_ERR_SCHED_LOCKED,
    /// The semaphore overflowed
    OS_ERR_SEM_OVF,

    /// The task create operation was called from an ISR
    OS_ERR_TASK_CREATE_ISR,
    /// The task delete operation failed
    OS_ERR_TASK_DEL,
    /// The task delete operation failed because the task is idle
    OS_ERR_TASK_DEL_IDLE,
    /// The task delete operation was requested
    OS_ERR_TASK_DEL_REQ,
    /// The task delete operation was called from an ISR
    OS_ERR_TASK_DEL_ISR,
    /// The task name is too long
    OS_ERR_TASK_NAME_TOO_LONG,
    /// No more TCBs are available
    OS_ERR_TASK_NO_MORE_TCB,
    /// The task does not exist
    OS_ERR_TASK_NOT_EXIST,
    /// The task is not suspended
    OS_ERR_TASK_NOT_SUSPENDED,
    /// The task option is invalid
    OS_ERR_TASK_OPT,
    /// The task resume priority is invalid
    OS_ERR_TASK_RESUME_PRIO,
    /// The task suspend operation failed because the task is idle
    OS_ERR_TASK_SUSPEND_IDLE,
    /// The task suspend operation failed because the priority is invalid
    OS_ERR_TASK_SUSPEND_PRIO,
    /// The task is waiting
    OS_ERR_TASK_WAITING,

    /// The time is not a delay
    OS_ERR_TIME_NOT_DLY,
    /// The minutes are invalid
    OS_ERR_TIME_INVALID_MINUTES,
    /// The seconds are invalid
    OS_ERR_TIME_INVALID_SECONDS,
    /// The milliseconds are invalid
    OS_ERR_TIME_INVALID_MS,
    /// The delay is zero
    OS_ERR_TIME_ZERO_DLY,
    /// The delay operation was called from an ISR
    OS_ERR_TIME_DLY_ISR,

    /// The memory partition is invalid
    OS_ERR_MEM_INVALID_PART,
    /// The memory block size is invalid
    OS_ERR_MEM_INVALID_BLKS,
    /// The memory size is invalid
    OS_ERR_MEM_INVALID_SIZE,
    /// There are no free memory blocks
    OS_ERR_MEM_NO_FREE_BLKS,
    /// The memory is full
    OS_ERR_MEM_FULL,
    /// The memory partition block is invalid
    OS_ERR_MEM_INVALID_PBLK,
    /// The memory partition memory is invalid
    OS_ERR_MEM_INVALID_PMEM,
    /// The memory partition data is invalid
    OS_ERR_MEM_INVALID_PDATA,
    /// The memory address is invalid
    OS_ERR_MEM_INVALID_ADDR,
    /// The memory name is too long
    OS_ERR_MEM_NAME_TOO_LONG,

    /// The task is not the mutex owner
    OS_ERR_NOT_MUTEX_OWNER,

    /// The flag group is invalid
    OS_ERR_FLAG_INVALID_PGRP,
    /// The flag wait type is invalid
    OS_ERR_FLAG_WAIT_TYPE,
    /// The flag is not ready
    OS_ERR_FLAG_NOT_RDY,
    /// The flag option is invalid
    OS_ERR_FLAG_INVALID_OPT,
    /// The flag group is depleted
    OS_ERR_FLAG_GRP_DEPLETED,
    /// The flag name is too long
    OS_ERR_FLAG_NAME_TOO_LONG,

    /// The PCP is lower than the current PCP
    OS_ERR_PCP_LOWER,

    /// The timer delay is invalid
    OS_ERR_TMR_INVALID_DLY,
    /// The timer period is invalid
    OS_ERR_TMR_INVALID_PERIOD,
    /// The timer option is invalid
    OS_ERR_TMR_INVALID_OPT,
    /// The timer name is invalid
    OS_ERR_TMR_INVALID_NAME,
    /// The timer is not available
    OS_ERR_TMR_NON_AVAIL,
    /// The timer is inactive
    OS_ERR_TMR_INACTIVE,
    /// The timer destination is invalid
    OS_ERR_TMR_INVALID_DEST,
    /// The timer type is invalid
    OS_ERR_TMR_INVALID_TYPE,
    /// The timer is invalid
    OS_ERR_TMR_INVALID,
    /// The timer operation was called from an ISR
    OS_ERR_TMR_ISR,
    /// The timer name is too long
    OS_ERR_TMR_NAME_TOO_LONG,
    /// The timer state is invalid
    OS_ERR_TMR_INVALID_STATE,
    /// The timer is stopped
    OS_ERR_TMR_STOPPED,
    /// The timer has no callback function
    OS_ERR_TMR_NO_CALLBACK,

    /// No more IDs are available
    OS_ERR_NO_MORE_ID_AVAIL,

    /// No more TLS slots are available
    OS_ERR_TLS_NO_MORE_AVAIL,
    /// The TLS ID is invalid
    OS_ERR_TLS_ID_INVALID,
    /// The TLS is not enabled
    OS_ERR_TLS_NOT_EN,
    /// The TLS destructor is already assigned
    OS_ERR_TLS_DESTRUCT_ASSIGNED,
    /// The operating system is not running
    OS_ERR_OS_NOT_RUNNING,
}

/*
*********************************************************************************************************
*                                         OS_PRIO
*********************************************************************************************************
*/

#[cfg(feature = "OS_PRIO_LESS_THAN_64")]
pub type OsPrio = INT8U;

#[cfg(feature = "OS_PRIO_LESS_THAN_256")]
pub type OS_PRIO = INT16U;

// if use both of the features, there will be an error
#[cfg(not(any(feature = "OS_PRIO_LESS_THAN_64", feature = "OS_PRIO_LESS_THAN_256")))]
pub type OsPrio = INT8U;
// there will be an error if both features is active
#[cfg(all(feature = "OS_PRIO_LESS_THAN_64", feature = "OS_PRIO_LESS_THAN_256"))]
compile_error!("You may not enable both `OS_PRIO_LESS_THAN_64` and `OS_PRIO_LESS_THAN_256` features.");

/*
*********************************************************************************************************
*                                         EVENT CONTROL BLOCK
*********************************************************************************************************
*/

/// the ref of ECB
#[cfg(feature="OS_EVENT_EN")]
pub struct OsEventRef{
    ptr:NonNull<OsEvent>,
}

/// the value of osevent_ptr, which can be a message or a queue structure
#[cfg(feature="OS_EVENT_EN")]
pub enum ECBPtr{
    Event(OsEventRef),
}

// only need to expose to current crate
#[cfg(feature="OS_EVENT_EN")]
pub(crate) struct OsEvent{
    osevent_type:INT8U,                      /* Type of event control block (see OS_EVENT_TYPE_xxxx)    */
    osevent_ptr:Option<ECBPtr>,            /* Pointer to message or queue structure                   */
    osevent_cnt:INT16U,                      /* Semaphore Count (not used if other EVENT type)          */
    osevent_grp:OsPrio,                     /* Group corresponding to tasks waiting for event to occur */
    osevent_tbl:[OsPrio;OS_EVENT_TBL_SIZE as usize], /* List of tasks waiting for event to occur                */
    #[cfg(feature="OS_EVENT_NAME_EN")]
    // the name of the event
    osevent_name:str
}


/*
*********************************************************************************************************
*                                      EVENT FLAGS CONTROL BLOCK
*********************************************************************************************************
*/

/*
*********************************************************************************************************
*                                        MESSAGE MAILBOX DATA
*********************************************************************************************************
*/

#[cfg(feature="OS_MBOX_EN")]
pub struct OS_MBOX_DATA{
    OSMboxMsg:Option<OsEvent>, /* Pointer to message in mailbox                          */
    OSMboxAccept:INT8U,         /* Indicates if the message has been read                 */
}

/*
*********************************************************************************************************
*                                  MEMORY PARTITION DATA STRUCTURES
*********************************************************************************************************
*/
#[cfg(all(feature = "OS_MEM_EN",feature = "OS_MAX_MEM_PART_EN"))]
pub struct OS_MEM{
    
}
    