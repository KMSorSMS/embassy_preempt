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
*                                             ERROR CODES
*********************************************************************************************************
*/

#[derive(PartialEq)]
#[repr(align(8))]
pub enum OsErrState {
OS_ERR_NONE,
OS_ERR_EVENT_TYPE,
OS_ERR_PEND_ISR,
OS_ERR_POST_NULL_PTR,
OS_ERR_PEVENT_NULL,
OS_ERR_POST_ISR,
OS_ERR_QUERY_ISR,
OS_ERR_INVALID_OPT,
OS_ERR_ID_INVALID,
OS_ERR_PDATA_NULL,

OS_ERR_TIMEOUT,
OS_ERR_EVENT_NAME_TOO_LONG,
OS_ERR_PNAME_NULL,
OS_ERR_PEND_LOCKED,
OS_ERR_PEND_ABORT,
OS_ERR_DEL_ISR,
OS_ERR_CREATE_ISR,
OS_ERR_NAME_GET_ISR,
OS_ERR_NAME_SET_ISR,
OS_ERR_ILLEGAL_CREATE_RUN_TIME,

OS_ERR_MBOX_FULL,
OS_ERR_ILLEGAL_DEL_RUN_TIME,

OS_ERR_Q_FULL,
OS_ERR_Q_EMPTY,

OS_ERR_PRIO_EXIST,
OS_ERR_PRIO,
OS_ERR_PRIO_INVALID,

OS_ERR_SCHED_LOCKED,
OS_ERR_SEM_OVF,

OS_ERR_TASK_CREATE_ISR,
OS_ERR_TASK_DEL,
OS_ERR_TASK_DEL_IDLE,
OS_ERR_TASK_DEL_REQ,
OS_ERR_TASK_DEL_ISR,
OS_ERR_TASK_NAME_TOO_LONG,
OS_ERR_TASK_NO_MORE_TCB,
OS_ERR_TASK_NOT_EXIST,
OS_ERR_TASK_NOT_SUSPENDED,
OS_ERR_TASK_OPT,
OS_ERR_TASK_RESUME_PRIO,
OS_ERR_TASK_SUSPEND_IDLE,
OS_ERR_TASK_SUSPEND_PRIO,
OS_ERR_TASK_WAITING,

OS_ERR_TIME_NOT_DLY,
OS_ERR_TIME_INVALID_MINUTES,
OS_ERR_TIME_INVALID_SECONDS,
OS_ERR_TIME_INVALID_MS,
OS_ERR_TIME_ZERO_DLY,
OS_ERR_TIME_DLY_ISR,

OS_ERR_MEM_INVALID_PART,
OS_ERR_MEM_INVALID_BLKS,
OS_ERR_MEM_INVALID_SIZE,
OS_ERR_MEM_NO_FREE_BLKS,
OS_ERR_MEM_FULL,
OS_ERR_MEM_INVALID_PBLK,
OS_ERR_MEM_INVALID_PMEM,
OS_ERR_MEM_INVALID_PDATA,
OS_ERR_MEM_INVALID_ADDR,
OS_ERR_MEM_NAME_TOO_LONG,

OS_ERR_NOT_MUTEX_OWNER,

OS_ERR_FLAG_INVALID_PGRP,
OS_ERR_FLAG_WAIT_TYPE,
OS_ERR_FLAG_NOT_RDY,
OS_ERR_FLAG_INVALID_OPT,
OS_ERR_FLAG_GRP_DEPLETED,
OS_ERR_FLAG_NAME_TOO_LONG,

OS_ERR_PCP_LOWER,

OS_ERR_TMR_INVALID_DLY,
OS_ERR_TMR_INVALID_PERIOD,
OS_ERR_TMR_INVALID_OPT,
OS_ERR_TMR_INVALID_NAME,
OS_ERR_TMR_NON_AVAIL,
OS_ERR_TMR_INACTIVE,
OS_ERR_TMR_INVALID_DEST,
OS_ERR_TMR_INVALID_TYPE,
OS_ERR_TMR_INVALID,
OS_ERR_TMR_ISR,
OS_ERR_TMR_NAME_TOO_LONG,
OS_ERR_TMR_INVALID_STATE,
OS_ERR_TMR_STOPPED,
OS_ERR_TMR_NO_CALLBACK,

OS_ERR_NO_MORE_ID_AVAIL,

OS_ERR_TLS_NO_MORE_AVAIL,
OS_ERR_TLS_ID_INVALID,
OS_ERR_TLS_NOT_EN,
OS_ERR_TLS_DESTRUCT_ASSIGNED,
OS_ERR_OS_NOT_RUNNING,
}