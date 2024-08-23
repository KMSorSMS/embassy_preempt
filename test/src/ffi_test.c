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

// 阻塞函数
void delay(int count){
    for(int i = 0; i < count; i++){
        for(int j = 0; j < 10000; j++);
    }
}
// 定义任务函数
void task(void* p_arg){
    LED2_ON();
    delay(300);
    LED2_OFF();
    delay(300);
}

// 点亮LED2，对应PA5，操作GPIO口
void LED2_Init(void)
{
    // 先设置GPIO的模式，对应GPIO的MODER寄存器
    GPIOA->MODER &= ~0x00000C00; // 先清零
    GPIOA->MODER |= 0x00000400;  // 设置为01，即输出模式
    // 设置GPIO的输出类型，对应GPIO的OTYPER寄存器
    GPIOA->OTYPER &= ~0x00000020; // 先清零,不过PUSH-PULL模式置为就是为0
    // 设置GPIO的输出速度，对应GPIO的OSPEEDR寄存器，这里我们设置为高速吧
    GPIOA->OSPEEDR &= ~0x00000C00; // 先清零
    GPIOA->OSPEEDR |= 0x00000800;  // 设置为10，即高速
    // 设置GPIO的上下拉电阻，对应GPIO的PUPDR寄存器，这里我们设置为没有上下拉
    GPIOA->PUPDR &= ~0x00000C00; // 先清零
    // 最后设置GPIO的输出值，对应GPIO的ODR寄存器，这里我们设置为高电平
    GPIOA->ODR |= 0x00000020; // 设置为1，即高电平
}
// 写一个时钟初始化的函数，配置为HSE，PLLM为4，PLLN为84，PLLP分频为2，PLLQ分频为4，还有AHB的地方分频为1 ，得到主频为84Mhz
void RCC_Configuration(void)
{
    // 先把PLL和PLL2S disable了
    RCC->CR &= ~0x05000000;
    // 先该各个分频系数,并且选择PLLSRC，把HSE设置为PLL的输入源
    RCC->PLLCFGR = 0b00000100010000000001010100000100; // 0x4401504的二进制是：0000 0100 0100 0000 0001 0101 0000 0100
    // 上面的配置是：PLLM=4, PLLN=84, PLLP=2, PLLQ=4，并且设置HSE为PLL的输入源
    // 设置AHB的分频系数为1
    RCC->CFGR &= ~0xF0;
    // 设置APB1的分频系数为2，APB2的分频系数为1
    RCC->CFGR |= 0x1000;
    RCC->CFGR &= ~0xE000;
    // 设置启动HSE，开启PLL和PLL2S
    RCC->CR |= 0b00000101000000010000000000000000; // 0x5010000
    // 加入保护代码，检查HSE和PLL、PLL2S的启动状态：
    while ((RCC->CR & 0x00020000) == 0); // 等待HSE启动成功
    while ((RCC->CR & 0x02000000) == 0); // 等待PLL启动成功
    while ((RCC->CR & 0x08000000) == 0); // 等待PLL2S启动成功
    // HSE启动成功后，使能FLASH预存取缓冲区
    FLASH->ACR |= FLASH_ACR_PRFTEN;
    // 设置FLASH的延时周期
    FLASH->ACR |= FLASH_ACR_LATENCY_2WS;
    // 更改系统的时钟源为PLL
    RCC->CFGR |= 0x00000002;
    // 关闭HSI
    RCC->CR &= ~0x00000001;
    // 接下来我们需要设置外设的时钟使能，点灯的时候用到AHB1上的GPIOA
    RCC->AHB1ENR |= 0x00000001;
}


int main(){
    // 时钟初始化
    RCC_Configuration();
    LED2_Init();
    LED2_OFF();
    OSInit();
    // 调用Rust函数
    OSTaskCreate(task, 0, 0, 0);
    OSStart();
    while(1);
}
