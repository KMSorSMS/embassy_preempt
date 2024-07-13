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
*                                     rewrite by liam and noah
*
*********************************************************************************************************
*/

/*
*********************************************************************************************************
*
*                                            CORE FUNCTIONS
*
* Filename : os_core.rs
* Version  : V2.93.01
*********************************************************************************************************
*/

/*
*********************************************************************************************************
*                                 The mods that os_core.rs depends on
*********************************************************************************************************
*/

use crate::port::*;

/*
*********************************************************************************************************
*                                      PRIORITY RESOLUTION TABLE
*
* Note: Index into table is bit pattern to resolve highest priority
*       Indexed value corresponds to highest priority bit position (i.e. 0..7)
*********************************************************************************************************
*/

#[allow(unused)]
const OS_UNMAP_TBL:[INT8U;256] = [
    0, 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0, /* 0x00 to 0x0F                   */
    4, 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0, /* 0x10 to 0x1F                   */
    5, 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0, /* 0x20 to 0x2F                   */
    4, 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0, /* 0x30 to 0x3F                   */
    6, 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0, /* 0x40 to 0x4F                   */
    4, 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0, /* 0x50 to 0x5F                   */
    5, 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0, /* 0x60 to 0x6F                   */
    4, 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0, /* 0x70 to 0x7F                   */
    7, 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0, /* 0x80 to 0x8F                   */
    4, 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0, /* 0x90 to 0x9F                   */
    5, 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0, /* 0xA0 to 0xAF                   */
    4, 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0, /* 0xB0 to 0xBF                   */
    6, 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0, /* 0xC0 to 0xCF                   */
    4, 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0, /* 0xD0 to 0xDF                   */
    5, 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0, /* 0xE0 to 0xEF                   */
    4, 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0  /* 0xF0 to 0xFF                   */
];

/*
*********************************************************************************************************
*                        GET THE NAME OF A SEMAPHORE, MUTEX, MAILBOX or QUEUE
*
* Description: This function is used to obtain the name assigned to a semaphore, mutex, mailbox or queue.
*
* Arguments  : pevent    is a pointer to the event group.  'pevent' can point either to a semaphore,
*                        a mutex, a mailbox or a queue.  Where this function is concerned, the actual
*                        type is irrelevant.
*
*              pname     is a pointer to a pointer to an ASCII string that will receive the name of the semaphore,
*                        mutex, mailbox or queue.
*
*              perr      is a pointer to an error code that can contain one of the following values:
*
*                        OS_ERR_NONE                if the name was copied to 'pname'
*                        OS_ERR_EVENT_TYPE          if 'pevent' is not pointing to the proper event
*                                                   control block type.
*                        OS_ERR_PNAME_NULL          You passed a NULL pointer for 'pname'
*                        OS_ERR_PEVENT_NULL         if you passed a NULL pointer for 'pevent'
*                        OS_ERR_NAME_GET_ISR        if you are trying to call this function from an ISR
*
* Returns    : The length of the string or 0 if the 'pevent' is a NULL pointer.
*********************************************************************************************************
*/

/// This function is used to obtain the name assigned to a semaphore, mutex, mailbox or queue.
#[cfg(all(feature = "OS_EVENT_EN", feature = "OS_EVENT_NAME_EN"))]
pub fn os_event_nameget(){}

/*
*********************************************************************************************************
*                        ASSIGN A NAME TO A SEMAPHORE, MUTEX, MAILBOX or QUEUE
*
* Description: This function assigns a name to a semaphore, mutex, mailbox or queue.
*
* Arguments  : pevent    is a pointer to the event group.  'pevent' can point either to a semaphore,
*                        a mutex, a mailbox or a queue.  Where this function is concerned, it doesn't
*                        matter the actual type.
*
*              pname     is a pointer to an ASCII string that will be used as the name of the semaphore,
*                        mutex, mailbox or queue.
*
*              perr      is a pointer to an error code that can contain one of the following values:
*
*                        OS_ERR_NONE                if the requested task is resumed
*                        OS_ERR_EVENT_TYPE          if 'pevent' is not pointing to the proper event
*                                                   control block type.
*                        OS_ERR_PNAME_NULL          You passed a NULL pointer for 'pname'
*                        OS_ERR_PEVENT_NULL         if you passed a NULL pointer for 'pevent'
*                        OS_ERR_NAME_SET_ISR        if you called this function from an ISR
*
* Returns    : None
*********************************************************************************************************
*/

/// This function assigns a name to a semaphore, mutex, mailbox or queue.
#[cfg(all(feature = "OS_EVENT_EN", feature = "OS_EVENT_NAME_EN"))]
pub fn os_event_nameset(){}

/*
*********************************************************************************************************
*                                           INITIALIZATION
*
* Description: This function is used to initialize the internals of uC/OS-II and MUST be called prior to
*              creating any uC/OS-II object and, prior to calling OSStart().
*
* Arguments  : none
*
* Returns    : none
*********************************************************************************************************
*/
/// This function is used to initialize the internals of uC/OS-II and MUST be called
/// prior to creating any uC/OS-II object and, prior to calling OSStart().
pub fn os_init() {}
// info!("if os is running in os_init? {}", unsafe {
//     uc_thread::os_core::OS_IS_RUNNING
// });
// OSInitHookBegin();
// OS_InitMisc();
// OS_InitRdyList();
// OS_InitTCBList();
// OS_InitEventList();
// OS_MemInit();
// OS_QInit();
// OS_InitTaskIdle();

/*
*********************************************************************************************************
*                                              ENTER ISR
*
* Description: This function is used to notify uC/OS-II that you are about to service an interrupt
*              service routine (ISR).  This allows uC/OS-II to keep track of interrupt nesting and thus
*              only perform rescheduling at the last nested ISR.
*
* Arguments  : none
*
* Returns    : none
*
* Notes      : 1) This function should be called with interrupts already disabled
*              2) Your ISR can directly increment OSIntNesting without calling this function because
*                 OSIntNesting has been declared 'global'.
*              3) You MUST still call OSIntExit() even though you increment OSIntNesting directly.
*              4) You MUST invoke OSIntEnter() and OSIntExit() in pair.  In other words, for every call
*                 to OSIntEnter() at the beginning of the ISR you MUST have a call to OSIntExit() at the
*                 end of the ISR.
*              5) You are allowed to nest interrupts up to 255 levels deep.
*              6) I removed the OS_ENTER_CRITICAL() and OS_EXIT_CRITICAL() around the increment because
*                 OSIntEnter() is always called with interrupts disabled.
*********************************************************************************************************
*/

/// This function is used to notify uC/OS-II that you are about to service 
/// an interrupt service routine (ISR).  This allows uC/OS-II to keep track 
/// of interrupt nesting and thus only perform rescheduling at the last nested ISR.
pub fn os_int_enter(){}

/*
*********************************************************************************************************
*                                              EXIT ISR
*
* Description: This function is used to notify uC/OS-II that you have completed servicing an ISR.  When
*              the last nested ISR has completed, uC/OS-II will call the scheduler to determine whether
*              a new, high-priority task, is ready to run.
*
* Arguments  : none
*
* Returns    : none
*
* Notes      : 1) You MUST invoke OSIntEnter() and OSIntExit() in pair.  In other words, for every call
*                 to OSIntEnter() at the beginning of the ISR you MUST have a call to OSIntExit() at the
*                 end of the ISR.
*              2) Rescheduling is prevented when the scheduler is locked (see OS_SchedLock())
*********************************************************************************************************
*/

/// This function is used to notify uC/OS-II that you have completed servicing 
/// an ISR.  When the last nested ISR has completed, uC/OS-II will call the 
/// scheduler to determine whether a new, high-priority task, is ready to run.
pub fn os_int_exit(){}

/*
*********************************************************************************************************
*                                         PREVENT SCHEDULING
*
* Description: This function is used to prevent rescheduling to take place.  This allows your application
*              to prevent context switches until you are ready to permit context switching.
*
* Arguments  : none
*
* Returns    : none
*
* Notes      : 1) You MUST invoke OSSchedLock() and OSSchedUnlock() in pair.  In other words, for every
*                 call to OSSchedLock() you MUST have a call to OSSchedUnlock().
*********************************************************************************************************
*/

/// This function is used to prevent rescheduling to take place.
/// This allows your application to prevent context switches until 
/// you are ready to permit context switching.
#[cfg(feature="OS_SCHED_LOCK_EN")]
pub fn os_sched_lock(){}

/*
*********************************************************************************************************
*                                          ENABLE SCHEDULING
*
* Description: This function is used to re-allow rescheduling.
*
* Arguments  : none
*
* Returns    : none
*
* Notes      : 1) You MUST invoke OSSchedLock() and OSSchedUnlock() in pair.  In other words, for every
*                 call to OSSchedLock() you MUST have a call to OSSchedUnlock().
*********************************************************************************************************
*/

/// This function is used to re-allow rescheduling.
#[cfg(feature="OS_SCHED_LOCK_EN")]
pub fn os_sched_unlock(){}

/*
*********************************************************************************************************
*                                         START MULTITASKING
*
* Description: This function is used to start the multitasking process which lets uC/OS-II manages the
*              task that you have created.  Before you can call OSStart(), you MUST have called OSInit()
*              and you MUST have created at least one task.
*
* Arguments  : none
*
* Returns    : none
*
* Note       : OSStartHighRdy() MUST:
*                 a) Call OSTaskSwHook() then,
*                 b) Set OSRunning to OS_TRUE.
*                 c) Load the context of the task pointed to by OSTCBHighRdy.
*                 d_ Execute the task.
*********************************************************************************************************
*/

/// This function is used to start the multitasking process which lets 
/// uC/OS-II manages the task that you have created.  Before you can call 
/// OSStart(), you MUST have called OSInit() and you MUST have created at 
/// least one task.
pub fn os_start(){}

/*
*********************************************************************************************************
*                                         PROCESS SYSTEM TICK
*
* Description: This function is used to signal to uC/OS-II the occurrence of a 'system tick' (also known
*              as a 'clock tick').  This function should be called by the ticker ISR but, can also be
*              called by a high priority task.
*
* Arguments  : none
*
* Returns    : none
*********************************************************************************************************
*/

/// This function is used to signal to uC/OS-II the occurrence of a 'system tick' 
/// (also known as a 'clock tick').  This function should be called by the ticker 
/// ISR but, can also be called by a high priority task.
pub fn os_timetick(){}

/*
*********************************************************************************************************
*                                             GET VERSION
*
* Description: This function is used to return the version number of uC/OS-II.  The returned value
*              corresponds to uC/OS-II's version number multiplied by 10000.  In other words, version
*              2.01.00 would be returned as 20100.
*
* Arguments  : none
*
* Returns    : The version number of uC/OS-II multiplied by 10000.
*********************************************************************************************************
*/

/// This function is used to return the version number of uC/OS-II.
/// The returned value corresponds to uC/OS-II's version number multiplied by 10000.
/// In other words, version 2.01.00 would be returned as 20100.
pub fn os_version()->INT16U{
    return 0;
}

/*
*********************************************************************************************************
*                                           DUMMY FUNCTION
*
* Description: This function doesn't do anything.  It is called by OSTaskDel().
*
* Arguments  : none
*
* Returns    : none
*********************************************************************************************************
*/

/// This function doesn't do anything.  It is called by OSTaskDel().
#[cfg(feature="OS_TASK_DEL_EN")]
pub fn os_dummy() {}

/*
*********************************************************************************************************
*                           MAKE TASK READY TO RUN BASED ON EVENT OCCURING
*
* Description: This function is called by other uC/OS-II services and is used to ready a task that was
*              waiting for an event to occur.
*
* Arguments  : pevent      is a pointer to the event control block corresponding to the event.
*
*              pmsg        is a pointer to a message.  This pointer is used by message oriented services
*                          such as MAILBOXEs and QUEUEs.  The pointer is not used when called by other
*                          service functions.
*
*              msk         is a mask that is used to clear the status byte of the TCB.  For example,
*                          OSSemPost() will pass OS_STAT_SEM, OSMboxPost() will pass OS_STAT_MBOX etc.
*
*              pend_stat   is used to indicate the readied task's pending status:
*
*                          OS_STAT_PEND_OK      Task ready due to a post (or delete), not a timeout or
*                                               an abort.
*                          OS_STAT_PEND_ABORT   Task ready due to an abort.
*
* Returns    : none
*
* Note       : This function is INTERNAL to uC/OS-II and your application should not call it.
*********************************************************************************************************
*/

/// This function is called by other uC/OS-II services and is used to ready 
/// a task that was waiting for an event to occur.
#[cfg(feature="OS_EVENT_EN")]
pub fn os_event_task_rdy()->INT8U{
    return 0;
}

/*
*********************************************************************************************************
*                                  MAKE TASK WAIT FOR EVENT TO OCCUR
*
* Description: This function is called by other uC/OS-II services to suspend a task because an event has
*              not occurred.
*
* Arguments  : pevent   is a pointer to the event control block for which the task will be waiting for.
*
* Returns    : none
*
* Note       : This function is INTERNAL to uC/OS-II and your application should not call it.
*********************************************************************************************************
*/

/// This function is called by other uC/OS-II services to suspend a task 
/// because an event has not occurred.
#[cfg(feature="OS_EVENT_EN")]
pub fn os_event_task_wait(){}

/*
*********************************************************************************************************
*                                  REMOVE TASK FROM EVENT WAIT LIST
*
* Description: Remove a task from an event's wait list.
*
* Arguments  : ptcb     is a pointer to the task to remove.
*
*              pevent   is a pointer to the event control block.
*
* Returns    : none
*
* Note       : This function is INTERNAL to uC/OS-II and your application should not call it.
*********************************************************************************************************
*/

/// Remove a task from an event's wait list.
#[cfg(feature="OS_EVENT_EN")]
pub fn os_event_task_remove(){}

/*
*********************************************************************************************************
*                             INITIALIZE EVENT CONTROL BLOCK'S WAIT LIST
*
* Description: This function is called by other uC/OS-II services to initialize the event wait list.
*
* Arguments  : pevent    is a pointer to the event control block allocated to the event.
*
* Returns    : none
*
* Note       : This function is INTERNAL to uC/OS-II and your application should not call it.
*********************************************************************************************************
*/

/// This function is called by other uC/OS-II services to initialize the event wait list.
#[cfg(feature="OS_EVENT_EN")]
pub fn os_event_wait_list_init(){}

/*
*********************************************************************************************************
*                                             INITIALIZATION
*                           INITIALIZE THE FREE LIST OF EVENT CONTROL BLOCKS
*
* Description: This function is called by OSInit() to initialize the free list of event control blocks.
*
* Arguments  : none
*
* Returns    : none
*********************************************************************************************************
*/

#[allow(unused)]
fn os_init_event_list(){}

/*
*********************************************************************************************************
*                                             INITIALIZATION
*                                    INITIALIZE MISCELLANEOUS VARIABLES
*
* Description: This function is called by OSInit() to initialize miscellaneous variables.
*
* Arguments  : none
*
* Returns    : none
*********************************************************************************************************
*/

#[allow(unused)]
fn os_init_misc(){}

/*
*********************************************************************************************************
*                                             INITIALIZATION
*                                       INITIALIZE THE READY LIST
*
* Description: This function is called by OSInit() to initialize the Ready List.
*
* Arguments  : none
*
* Returns    : none
*********************************************************************************************************
*/

#[allow(unused)]
fn os_init_rdy_list(){}

/*
*********************************************************************************************************
*                                             INITIALIZATION
*                                         CREATING THE IDLE TASK
*
* Description: This function creates the Idle Task.
*
* Arguments  : none
*
* Returns    : none
*********************************************************************************************************
*/

#[allow(unused)]
fn os_init_task_idle(){}

/*
*********************************************************************************************************
*                                             INITIALIZATION
*                            INITIALIZE THE FREE LIST OF TASK CONTROL BLOCKS
*
* Description: This function is called by OSInit() to initialize the free list of OS_TCBs.
*
* Arguments  : none
*
* Returns    : none
*********************************************************************************************************
*/

#[allow(unused)]
fn os_init_tcblist(){}

/*
*********************************************************************************************************
*                                      CLEAR A SECTION OF MEMORY
*
* Description: This function is called by other uC/OS-II services to clear a contiguous block of RAM.
*
* Arguments  : pdest    is the start of the RAM to clear (i.e. write 0x00 to)
*
*              size     is the number of bytes to clear.
*
* Returns    : none
*
* Notes      : 1) This function is INTERNAL to uC/OS-II and your application should not call it.
*              2) Note that we can only clear up to 64K bytes of RAM.  This is not an issue because none
*                 of the uses of this function gets close to this limit.
*              3) The clear is done one byte at a time since this will work on any processor irrespective
*                 of the alignment of the destination.
*********************************************************************************************************
*/

/// This function is called by other uC/OS-II services to clear a contiguous block of RAM.
pub fn os_memclr(){}

/*
*********************************************************************************************************
*                                       COPY A BLOCK OF MEMORY
*
* Description: This function is called by other uC/OS-II services to copy a block of memory from one
*              location to another.
*
* Arguments  : pdest    is a pointer to the 'destination' memory block
*
*              psrc     is a pointer to the 'source'      memory block
*
*              size     is the number of bytes to copy.
*
* Returns    : none
*
* Notes      : 1) This function is INTERNAL to uC/OS-II and your application should not call it.  There is
*                 no provision to handle overlapping memory copy.  However, that's not a problem since this
*                 is not a situation that will happen.
*              2) Note that we can only copy up to 64K bytes of RAM
*              3) The copy is done one byte at a time since this will work on any processor irrespective
*                 of the alignment of the source and destination.
*********************************************************************************************************
*/

/// This function is called by other uC/OS-II services to copy a block of 
/// memory from one location to another.
pub fn os_memcopy(){}

/*
*********************************************************************************************************
*                                              SCHEDULER
*
* Description: This function is called by other uC/OS-II services to determine whether a new, high
*              priority task has been made ready to run.  This function is invoked by TASK level code
*              and is not used to reschedule tasks from ISRs (see OSIntExit() for ISR rescheduling).
*
* Arguments  : none
*
* Returns    : none
*
* Notes      : 1) This function is INTERNAL to uC/OS-II and your application should not call it.
*              2) Rescheduling is prevented when the scheduler is locked (see OS_SchedLock())
*********************************************************************************************************
*/

/// This function is called by other uC/OS-II services to determine whether a new, high 
/// priority task has been made ready to run.  This function is invoked by TASK level code
/// and is not used to reschedule tasks from ISRs (see OSIntExit() for ISR rescheduling).
pub fn os_sched(){}

/*
*********************************************************************************************************
*                               FIND HIGHEST PRIORITY TASK READY TO RUN
*
* Description: This function is called by other uC/OS-II services to determine the highest priority task
*              that is ready to run.  The global variable 'OSPrioHighRdy' is changed accordingly.
*
* Arguments  : none
*
* Returns    : none
*
* Notes      : 1) This function is INTERNAL to uC/OS-II and your application should not call it.
*              2) Interrupts are assumed to be disabled when this function is called.
*********************************************************************************************************
*/

#[allow(unused)]
fn os_sched_new(){}

