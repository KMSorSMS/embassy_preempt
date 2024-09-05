#ifndef OS_CPU_H
#define OS_CPU_H

typedef unsigned char BOOLEAN;
typedef unsigned char INT8U;   /* Unsigned  8 bit quantity                           */
typedef signed char INT8S;     /* Signed    8 bit quantity                           */
typedef unsigned short INT16U; /* Unsigned 16 bit quantity                           */
typedef signed short INT16S;   /* Signed   16 bit quantity                           */
typedef unsigned int INT32U;   /* Unsigned 32 bit quantity                           */
typedef signed int INT32S;     /* Signed   32 bit quantity                           */
typedef float FP32;            /* Single precision floating point                    */
typedef double FP64;           /* Double precision floating point                    */

typedef unsigned int OS_STK;    /* Each stack entry is 32-bit wide                    */
typedef unsigned int OS_CPU_SR; /* Define size of CPU status register (PSR = 32 bits) */

#define OS_CRITICAL_METHOD 3u

#if OS_CRITICAL_METHOD == 3u /* See OS_CPU_A.ASM                                   */
OS_CPU_SR OS_CPU_SR_Save(OS_CPU_SR new_basepri);
void OS_CPU_SR_Restore(OS_CPU_SR cpu_sr);
#endif

#if OS_CRITICAL_METHOD == 3u
/* Save current BASEPRI priority lvl for exception... */
/* .. and set BASEPRI to CPU_CFG_KA_IPL_BOUNDARY      */
#define OS_ENTER_CRITICAL()                                                                \
    do                                                                                     \
    {                                                                                      \
        cpu_sr = OS_CPU_SR_Save(CPU_CFG_KA_IPL_BOUNDARY << (8u - CPU_CFG_NVIC_PRIO_BITS)); \
    } while (0)
/* Restore CPU BASEPRI priority level.                */
#define OS_EXIT_CRITICAL()         \
    do                             \
    {                              \
        OS_CPU_SR_Restore(cpu_sr); \
    } while (0)
#endif

#ifdef OS_CPU_GLOBALS
#define OS_CPU_EXT
#else
#define OS_CPU_EXT extern
#endif
// 设置堆栈段的生长方式：
#define OS_STK_GROWTH 1u /* Stack grows from HIGH to LOW memory on ARM         */

// 设置CPU的异常堆栈大小
#ifndef OS_CPU_EXCEPT_STK_SIZE
#define OS_CPU_EXCEPT_STK_SIZE 256u /* Default exception stack size is 256 OS_STK entries */
#endif

// 从而进一步的设置OS的CPU异常栈
OS_CPU_EXT OS_STK OS_CPU_ExceptStk[OS_CPU_EXCEPT_STK_SIZE];
OS_CPU_EXT OS_STK *OS_CPU_ExceptStkBase;

// 声明开始执行最高优先级函数
void OSStartHighRdy(void);
// 根据传入的cpu HSE主频频率以及宏定义设置的systick频率来设置cnts重装载值，并调用下面的OS_CPU_SysTickInit
void OS_CPU_SysTickInitFreq(INT32U cpu_freq);

// 声明systick中断初始化函数,主要是初始化优先级，设置重装载值，使能systick的timer并且使能触发中断systick
void OS_CPU_SysTickInit(INT32U cnts);

// SysTick中断处理函数
void OS_CPU_SysTickHandler(void);

void OSTaskSwHook(void);
void OSInitHookBegin(void);
void OSCtxSw(void);
void OSIntCtxSw(void);
void PendSV_Handler(void);

// OS上下文切换
#define OS_TASK_SW() OSCtxSw()

#endif
