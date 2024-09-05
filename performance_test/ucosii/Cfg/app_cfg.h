#define OS_TASK_TMR_PRIO (OS_LOWEST_PRIO - 2u)
// 不受内核管理的中断等级，使用 BOUNDARY 寄存器设置任务切换时的中断屏蔽
#ifndef CPU_CFG_KA_IPL_BOUNDARY
#define CPU_CFG_KA_IPL_BOUNDARY 4u
#endif

// 中断优先级寄存器可用位数，ARM 官方为 8bit 由厂家裁剪
#ifndef CPU_CFG_NVIC_PRIO_BITS
#define CPU_CFG_NVIC_PRIO_BITS 4u
#endif