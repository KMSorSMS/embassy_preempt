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

use core::sync::atomic::Ordering;

// use critical_section::Mutex;
// use core::cell::RefCell;
use os_cpu::*;

use crate::{cfg::OS_LOWEST_PRIO, os_time::OSTimerInit};
use crate::executor::GlobalSyncExecutor;
use crate::heap::stack_allocator::init_stack_allocator;
use crate::os_task::OSTaskCreate;
// use crate::os_q::OS_QInit;
use crate::port::*;
#[cfg(feature = "OS_TASK_REG_TBL_SIZE")]
use crate::ucosii::OSTaskRegNextAvailID;
use crate::ucosii::{OSCtxSwCtr, OSIdleCtr, OSIntNesting, OSLockNesting, OSRunning, OSTaskCtr, OSTime};

/*
*********************************************************************************************************
*                                      PRIORITY RESOLUTION TABLE
*
* Note: Index into table is bit pattern to resolve highest priority
*       Indexed value corresponds to highest priority bit position (i.e. 0..7)
*********************************************************************************************************
*/

#[allow(unused)]
const OSUnMapTbl: [INT8U; 256] = [
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
    4, 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0, /* 0xF0 to 0xFF                   */
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
pub fn OSEventNameGet() {}

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
pub fn OSEventNameSet() {}

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
pub fn OSInit() {
    OSInitHookBegin(); /* Call port specific initialization code   */

    // by noah: this func is no need to be called because we give the static var init val
    // OS_InitMisc();/* Initialize miscellaneous variables       */
    // by noah：this function is not called because we use lazy_static
    // OS_InitRdyList();/* Initialize the Ready List                */
    // by noah：this function is not called because we can not init TCB here
    // OS_InitTCBList(); /* Initialize the free list of OS_TCBs      */
    // to be done: For now, we just aim to implement the task module, so we will impl OS_InitEventList in the future
    // OS_InitEventList(); /* Initialize the free list of OS_EVENTs    */
    // #[cfg(all(feature="OS_MEM_EN",feature="OS_MAX_MEM_PART_EN"))]
    // OS_MemInit(); /* Initialize the memory manager            */
    // // to be done: For now, we just aim to implement the task module, so we will impl OS_InitEventList in the future
    // #[cfg(all(feature="OS_Q_EN",feature="OS_MAX_QS"))]
    // OS_QInit();

    // by noah: There is still no need to implement idle task
    // OS_InitTaskIdle();

    #[cfg(all(feature = "OS_FLAG_EN", feature = "OS_MAX_FLAGS"))]
    OS_FlagInit(); /* Initialize the event flag structures     */

    #[cfg(feature = "OS_TASK_STAT_EN")]
    OS_InitTaskStat(); /* Create the Statistic Task                */

    #[cfg(feature = "OS_TMR_EN")]
    OSTmr_Init(); /* Initialize the Timer Manager             */

    #[cfg(feature = "OS_CPU_HOOKS_EN")]
    OSInitHookEnd(); /* Call port specific init. code            */

    #[cfg(feature = "OS_DEBUG_EN")]
    OSDebugInit();
    OS_InitTaskIdle(); /* Create the Idle Task                     */
    // by liam: we need to init the stack allocator
    init_stack_allocator();
    // by noah：we need to init the Timer as the time driver
    OSTimerInit();
}

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
pub fn OSIntEnter() {}

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
pub fn OSIntExit() {}

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
#[cfg(feature = "OS_SCHED_LOCK_EN")]
pub fn OSSchedLock() {}

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
#[cfg(feature = "OS_SCHED_LOCK_EN")]
pub fn OSSchedUnlock() {}

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
// #[cfg(not(feature = "test"))]
pub fn OSStart() -> ! {
    use crate::heap::stack_allocator::INTERRUPT_STACK;

    extern "Rust" {
        fn set_int_change_2_psp(int_ptr: *mut u8);
    }
    // set OSRunning
    OSRunning.store(true, Ordering::Release);
    // before we step into the loop, we call set_int_change_2_psp(as part of the function of OSStartHighRdy in ucosii)
    // to change the stack pointer to program pointer and use psp
    let int_stk = INTERRUPT_STACK.exclusive_access();
    let int_ptr = int_stk.STK_REF.as_ptr() as *mut u8;
    drop(int_stk);
    unsafe {
        set_int_change_2_psp(int_ptr);
        // find the highest priority task in the ready queue
        critical_section::with(|_| GlobalSyncExecutor.as_ref().unwrap().set_highrdy());
        GlobalSyncExecutor.as_ref().unwrap().poll();
    }
}

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
/// fin this to use Timer
// by noah:we do not need the function because we use the Timer. Sysytem tick will be stopped when the MCU is in lower power mode
// pub fn OSTimeTick() {
//     // add the
//     OSTime.fetch_add(1, Ordering::Release);
// }

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
pub fn OSVersion() -> INT16U {
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
#[cfg(feature = "OS_TASK_DEL_EN")]
pub fn OS_Dummy() {}

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
#[cfg(feature = "OS_EVENT_EN")]
pub fn OS_EventTaskRdy() -> INT8U {
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
#[cfg(feature = "OS_EVENT_EN")]
pub fn OS_EventTaskWait() {}

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
#[cfg(feature = "OS_EVENT_EN")]
pub fn OS_EventTaskRemove() {}

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
#[cfg(feature = "OS_EVENT_EN")]
pub fn OS_EventWaitListInit() {}

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
fn OS_InitEventList() {}

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

// by noah: maybe we can use Ordering::Relaxed?
#[allow(unused)]
fn OS_InitMisc() {
    #[cfg(feature = "OS_TIME_GET_SET_EN")]
    OSTime.store(0, Ordering::Release); /* Clear the 32-bit system clock            */

    OSIntNesting.store(0, Ordering::Release); /* Clear the interrupt nesting counter     */
    OSLockNesting.store(0, Ordering::Release); /* Clear the scheduling lock counter        */

    OSTaskCtr.store(0, Ordering::Release); /* Clear the number of tasks                */

    OSRunning.store(false, Ordering::Release); /* Indicate that multitasking not started   */

    OSCtxSwCtr.store(0, Ordering::Release); /* Clear the context switch counter         */
    OSIdleCtr.store(0, Ordering::Release); /* Clear the 32-bit idle counter            */

    #[cfg(feature = "OS_TASK_STAT_EN")]
    OSIdleCtrRun.store(0, Ordering::Release);
    #[cfg(feature = "OS_TASK_STAT_EN")]
    OSIdleCtrMax.store(0, Ordering::Release);
    #[cfg(feature = "OS_TASK_STAT_EN")]
    OSStatRdy.store(false, Ordering::Release);

    #[cfg(feature = "OS_SAFETY_CRITICAL_IEC61508")]
    OSSafetyCriticalStartFlag.store(false, Ordering::Release);

    #[cfg(feature = "OS_TASK_REG_TBL_SIZE")]
    OSTaskRegNextAvailID.store(0, Ordering::Release);
}

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

// init the ready bit map
#[allow(unused)]
fn OS_InitRdyList() {
    /* Clear the ready list                     */
    // fix by liam: the rdy list is bounded with executor, we don't need this anymore
    // // check by liam: may be Ordering::relaxed as the os have not started
    // OSRdyGrp.store(0, Ordering::Release);

    // OSPrioCur.store(0, Ordering::Release);

    // OSPrioHighRdy.store(0, Ordering::Release);

    // // by noah: to init static var with type Mutex, we need a cs
    // critical_section::with(|cs|{
    //     // init the ready table
    //     OSRdyTbl.borrow_ref_mut(cs).iter_mut().for_each(|x| {
    //         *x = 0;// set the array element to 0
    //     });
    //     *(OSTCBCur.borrow_ref_mut(cs)) = OS_TCB_REF::default();
    //     *(OSTCBHighRdy.borrow_ref_mut(cs))= OS_TCB_REF::default();
    // })
}

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
// must use this function
fn OS_InitTaskIdle() {
    extern "Rust" {
        fn run_idle();
    }
    let idle_fn = |_args: *mut ()| -> ! {
        loop {
            unsafe {
                run_idle();
            }
        }
    };
    OSTaskCreate(idle_fn, 0 as *mut (), 0 as *mut usize, OS_LOWEST_PRIO);
}

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
fn OS_InitTCBList() {
    //
}

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
pub fn OS_MemClr() {}

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
pub fn OS_MemCopy() {}

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
pub fn OS_Sched() {}

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
fn OS_SchedNew() {}

/*
*********************************************************************************************************
*                               DETERMINE THE LENGTH OF AN ASCII STRING
*
* Description: This function is called by other uC/OS-II services to determine the size of an ASCII string
*              (excluding the NUL character).
*
* Arguments  : psrc     is a pointer to the string for which we need to know the size.
*
* Returns    : The size of the string (excluding the NUL terminating character)
*
* Notes      : 1) This function is INTERNAL to uC/OS-II and your application should not call it.
*              2) The string to check must be less than 255 characters long.
*********************************************************************************************************
*/

/// This function is called by other uC/OS-II services to determine the size of an ASCII string (excluding the NUL character)
pub fn OS_StrLen(_psrc: &str) -> INT8U {
    return 0;
}

/*
*********************************************************************************************************
*                                              IDLE TASK
*
* Description: This task is internal to uC/OS-II and executes whenever no other higher priority tasks
*              executes because they are ALL waiting for event(s) to occur.
*
* Arguments  : none
*
* Returns    : none
*
* Note(s)    : 1) OSTaskIdleHook() is called after the critical section to ensure that interrupts will be
*                 enabled for at least a few instructions.  On some processors (ex. Philips XA), enabling
*                 and then disabling interrupts didn't allow the processor enough time to have interrupts
*                 enabled before they were disabled again.  uC/OS-II would thus never recognize
*                 interrupts.
*              2) This hook has been added to allow you to do such things as STOP the CPU to conserve
*                 power.
*********************************************************************************************************
*/

#[allow(unused)]
fn OS_TaskIdle() {}

/*
*********************************************************************************************************
*                                        CHECK ALL TASK STACKS
*
* Description: This function is called by OS_TaskStat() to check the stacks of each active task.
*
* Arguments  : none
*
* Returns    : none
*********************************************************************************************************
*/

/// his function is called by OS_TaskStat() to check the stacks of each active task.
#[cfg(all(feature = "OS_TASK_STAT_STK_CHK_EN", feature = "OS_TASK_CREATE_EXT_EN"))]
pub fn OS_TaskStatStkChk() {}

/*
*********************************************************************************************************
*                                           INITIALIZE TCB
*
* Description: This function is internal to uC/OS-II and is used to initialize a Task Control Block when
*              a task is created (see OSTaskCreate() and OSTaskCreateExt()).
*
* Arguments  : prio          is the priority of the task being created
*
*              ptos          is a pointer to the task's top-of-stack assuming that the CPU registers
*                            have been placed on the stack.  Note that the top-of-stack corresponds to a
*                            'high' memory location is OS_STK_GROWTH is set to 1 and a 'low' memory
*                            location if OS_STK_GROWTH is set to 0.  Note that stack growth is CPU
*                            specific.
*
*              pbos          is a pointer to the bottom of stack.  A NULL pointer is passed if called by
*                            'OSTaskCreate()'.
*
*              id            is the task's ID (0..65535)
*
*              stk_size      is the size of the stack (in 'stack units').  If the stack units are INT8Us
*                            then, 'stk_size' contains the number of bytes for the stack.  If the stack
*                            units are INT32Us then, the stack contains '4 * stk_size' bytes.  The stack
*                            units are established by the #define constant OS_STK which is CPU
*                            specific.  'stk_size' is 0 if called by 'OSTaskCreate()'.
*
*              pext          is a pointer to a user supplied memory area that is used to extend the task
*                            control block.  This allows you to store the contents of floating-point
*                            registers, MMU registers or anything else you could find useful during a
*                            context switch.  You can even assign a name to each task and store this name
*                            in this TCB extension.  A NULL pointer is passed if called by OSTaskCreate().
*
*              opt           options as passed to 'OSTaskCreateExt()' or,
*                            0 if called from 'OSTaskCreate()'.
*
* Returns    : OS_ERR_NONE              if the call was successful
*              OS_ERR_TASK_NO_MORE_TCB  if there are no more free TCBs to be allocated and thus, the task
*                                       cannot be created.
*
* Note       : This function is INTERNAL to uC/OS-II and your application should not call it.
*********************************************************************************************************
*/
