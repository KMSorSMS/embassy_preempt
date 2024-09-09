#define OS_CPU_GLOBALS
#include "os_cpu.h"

#include <ucos_ii.h>

/*
*********************************************************************************************************
*                                          SYS TICK DEFINES
*********************************************************************************************************
*/

#define OS_CPU_CM_SYST_CSR (*((volatile INT32U *)0xE000E010uL))   /* SysTick Ctrl & Status Reg.                  */
#define OS_CPU_CM_SYST_RVR (*((volatile INT32U *)0xE000E014uL))   /* SysTick Reload  Value Reg.                  */
#define OS_CPU_CM_SYST_CVR (*((volatile INT32U *)0xE000E018uL))   /* SysTick Current Value Reg.                  */
#define OS_CPU_CM_SYST_CALIB (*((volatile INT32U *)0xE000E01CuL)) /* SysTick Cal     Value Reg.                  */
#define OS_CPU_CM_SCB_SHPRI3 (*((volatile INT32U *)0xE000ED20uL)) /* System Handlers 12 to 15 Prio.              */

#define OS_CPU_CM_SYST_CSR_COUNTFLAG 0x00010000uL /* Count flag.                                 */
#define OS_CPU_CM_SYST_CSR_CLKSOURCE 0x00000004uL /* Clock Source.                               */
#define OS_CPU_CM_SYST_CSR_TICKINT 0x00000002uL   /* Interrupt enable.                           */
#define OS_CPU_CM_SYST_CSR_ENABLE 0x00000001uL    /* Counter mode.                       */

INT32U OS_KA_BASEPRI_Boundary; /* Base Priority boundary.                              */

void OSTaskSwHook(void)
{
    // Do nothing
}

OS_STK *OSTaskStkInit(void (*task)(void *p_arg), void *p_arg, OS_STK *ptos, INT16U opt)
{
    OS_STK *p_stk;

    (void)opt;         /* 'opt' is not used, prevent warning                   */
    p_stk = ptos + 1u; /* Load stack pointer                                   */
                       /* Align the stack to 8-bytes.                          */
    p_stk = (OS_STK *)((OS_STK)(p_stk) & 0xFFFFFFF8u);
    /* Registers stacked as if auto-saved on exception      */
#if (OS_CPU_ARM_FP_EN > 0u) /* FPU auto-saved registers.                            */
    --p_stk;
    *(--p_stk) = (OS_STK)0x02000000u; /* FPSCR                                                */
                                      /* Initialize S0-S15 floating point registers           */
    *(--p_stk) = (OS_STK)0x41700000u; /* S15                                                  */
    *(--p_stk) = (OS_STK)0x41600000u; /* S14                                                  */
    *(--p_stk) = (OS_STK)0x41500000u;s /* S13                                                  */
    *(--p_stk) = (OS_STK)0x41400000u; /* S12                                                  */
    *(--p_stk) = (OS_STK)0x41300000u; /* S11                                                  */
    *(--p_stk) = (OS_STK)0x41200000u; /* S10                                                  */
    *(--p_stk) = (OS_STK)0x41100000u; /* S9                                                   */
    *(--p_stk) = (OS_STK)0x41000000u; /* S8                                                   */
    *(--p_stk) = (OS_STK)0x40E00000u; /* S7                                                   */
    *(--p_stk) = (OS_STK)0x40C00000u; /* S6                                                   */
    *(--p_stk) = (OS_STK)0x40A00000u; /* S5                                                   */
    *(--p_stk) = (OS_STK)0x40800000u; /* S4                                                   */
    *(--p_stk) = (OS_STK)0x40400000u; /* S3                                                   */
    *(--p_stk) = (OS_STK)0x40000000u; /* S2                                                   */
    *(--p_stk) = (OS_STK)0x3F800000u; /* S1                                                   */
    *(--p_stk) = (OS_STK)0x00000000u; /* S0                                                   */
#endif
    *(--p_stk) = (OS_STK)0x01000000uL;  /* xPSR                                                 */
    *(--p_stk) = (OS_STK)task;          /* Entry Point                                          */
    *(--p_stk) = (OS_STK)OS_TaskReturn; /* R14 (LR)                                             */
    *(--p_stk) = (OS_STK)0x12121212uL;  /* R12                                                  */
    *(--p_stk) = (OS_STK)0x03030303uL;  /* R3                                                   */
    *(--p_stk) = (OS_STK)0x02020202uL;  /* R2                                                   */
    *(--p_stk) = (OS_STK)0x01010101uL;  /* R1                                                   */
    *(--p_stk) = (OS_STK)p_arg;         /* R0 : argument                                        */

#if (OS_CPU_ARM_FP_EN > 0u)
    *(--p_stk) = (OS_STK)0xFFFFFFEDuL; /* R14: EXEC_RETURN; See Note 5                         */
#else
    *(--p_stk) = (OS_STK)0xFFFFFFFDuL; /* R14: EXEC_RETURN; See Note 5                         */
#endif
    /* Remaining registers saved on process stack           */
    *(--p_stk) = (OS_STK)0x11111111uL; /* R11                                                  */
    *(--p_stk) = (OS_STK)0x10101010uL; /* R10                                                  */
    *(--p_stk) = (OS_STK)0x09090909uL; /* R9                                                   */
    *(--p_stk) = (OS_STK)0x08080808uL; /* R8                                                    */
    *(--p_stk) = (OS_STK)0x07070707uL; /* R7                                                   */
    *(--p_stk) = (OS_STK)0x06060606uL; /* R6                                                   */
    *(--p_stk) = (OS_STK)0x05050505uL; /* R5                                                   */
    *(--p_stk) = (OS_STK)0x04040404uL; /* R4                                                   */

#if (OS_CPU_ARM_FP_EN > 0u)
    /* Initialize S16-S31 floating point registers          */
    *(--p_stk) = (OS_STK)0x41F80000u; /* S31                                                  */
    *(--p_stk) = (OS_STK)0x41F00000u; /* S30                                                  */
    *(--p_stk) = (OS_STK)0x41E80000u; /* S29                                                  */
    *(--p_stk) = (OS_STK)0x41E00000u; /* S28                                                  */
    *(--p_stk) = (OS_STK)0x41D80000u; /* S27                                                  */
    *(--p_stk) = (OS_STK)0x41D00000u; /* S26                                                  */
    *(--p_stk) = (OS_STK)0x41C80000u; /* S25                                                  */
    *(--p_stk) = (OS_STK)0x41C00000u; /* S24                                                  */
    *(--p_stk) = (OS_STK)0x41B80000u; /* S23                                                  */
    *(--p_stk) = (OS_STK)0x41B00000u; /* S22                                                  */
    *(--p_stk) = (OS_STK)0x41A80000u; /* S21                                                  */
    *(--p_stk) = (OS_STK)0x41A00000u; /* S20                                                  */
    *(--p_stk) = (OS_STK)0x41980000u; /* S19                                                  */
    *(--p_stk) = (OS_STK)0x41900000u; /* S18                                                  */
    *(--p_stk) = (OS_STK)0x41880000u; /* S17                                                  */
    *(--p_stk) = (OS_STK)0x41800000u; /* S16                                                  */
#endif

    return (p_stk);
}

void OSInitHookBegin(void)
{
    INT32U size;
    OS_STK *pstk;
    // 先把OS异常栈清空吧：
    pstk = &OS_CPU_ExceptStk[0];
    size = OS_CPU_EXCEPT_STK_SIZE;
    while (size > 0u)
    {
        size--;
        *pstk++ = (OS_STK)0;
    }
    // 接下来我们就可以初始化OS_CPU_ExceptStkBase了，它是指向OS_CPU_ExceptStk的栈顶的指针
    OS_CPU_ExceptStkBase = (OS_STK *)&OS_CPU_ExceptStk[OS_CPU_EXCEPT_STK_SIZE];
    // 下面& 0xFFFFFFF8是为了保证OS_CPU_ExceptStkBase是8的倍数，因为ARM的栈是8字节对齐的
    OS_CPU_ExceptStkBase = (OS_STK *)((OS_STK)(OS_CPU_ExceptStkBase) & 0xFFFFFFF8);

    OS_KA_BASEPRI_Boundary = (INT32U)(CPU_CFG_KA_IPL_BOUNDARY << (8u - CPU_CFG_NVIC_PRIO_BITS));
}

void OSTaskIdleHook(void)
{
    // Do nothing
}

// sysTick中断
void OS_CPU_SysTickHandler(void)
{
#if OS_CRITICAL_METHOD == 3u /* Allocate storage for CPU status register             */
    OS_CPU_SR cpu_sr;
#endif

    OS_ENTER_CRITICAL();
    OSIntEnter(); /* Tell uC/OS-II that we are starting an ISR            */
    OS_EXIT_CRITICAL();

    OSTimeTick(); /* Call uC/OS-II's OSTimeTick()                         */

    OSIntExit(); /* Tell uC/OS-II that we are leaving the ISR            */
}

void OS_CPU_SysTickInitFreq(INT32U cpu_freq)
{
    INT32U cnts;

    
    cnts = (cpu_freq / (INT32U)OS_TICKS_PER_SEC); /* Determine nbr SysTick cnts between two OS tick intr. */

    OS_CPU_SysTickInit(cnts);
}

void OS_CPU_SysTickInit(INT32U cnts)
{
    INT32U prio;
    INT32U basepri;

    /* Set BASEPRI boundary from the configuration.         */
    basepri = (INT32U)(CPU_CFG_KA_IPL_BOUNDARY << (8u - CPU_CFG_NVIC_PRIO_BITS));
    OS_CPU_CM_SYST_RVR = cnts - 1u; /* Set Reload register.                                 */

    /* Set SysTick handler prio.                            */
    prio = OS_CPU_CM_SCB_SHPRI3;
    prio &= 0x00FFFFFFu;
    prio |= (basepri << 24u);
    OS_CPU_CM_SCB_SHPRI3 = prio;

    // 这里把systick当前的值清零，然后启动systick
    OS_CPU_CM_SYST_CVR = 0u; /* Clear current SysTick counter.                      */
                             /* Enable timer.                                        */
    OS_CPU_CM_SYST_CSR |= OS_CPU_CM_SYST_CSR_CLKSOURCE | OS_CPU_CM_SYST_CSR_ENABLE;
    /* Enable timer interrupt.                              */
    OS_CPU_CM_SYST_CSR |= OS_CPU_CM_SYST_CSR_TICKINT;
}