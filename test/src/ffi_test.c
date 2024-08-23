#include "stm32f401xe.h"
// 声明OS_ERR_STATE
typedef enum {
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
}OS_ERR_STATE;

// 定义OS_STK
typedef unsigned int OS_STK;
// 声明Rust函数
extern OS_ERR_STATE OSTaskCreate(void (*fun_ptr)(void*), void* p_arg, OS_STK* ptos, unsigned short prio);
extern void OSInit();
extern void OSStart();
// 开灯操作，用宏定义
#define LED2_ON() GPIOA->ODR |= 0x00000020; // 设置为1，即高电平
// 关灯操作，用宏定义
#define LED2_OFF() GPIOA->ODR &= ~0x00000020; // 设置为0，即低电平

// 定义任务函数
void task(void* p_arg){
    LED2_ON();
}

int main(){
    LED2_ON();
    OSInit();
    // 调用Rust函数
    OSTaskCreate(task, 0, 0, 0);
    OSStart();
    while(1);
}