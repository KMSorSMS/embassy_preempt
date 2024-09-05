#include "main.h"
#include "NVIC.h"
#include "OS_stk.h"
#include "os_cpu.h"
#include "tools.h"
#include "ucos_ii.h"
#include "os_trace.h"

#define BLOCK_TIME 1

// 采用嵌入式汇编实现对标其他测试的阻塞delay函数
void delay(uint32_t time)
{
    // 延时函数，time 的单位约为 0.5s，使用汇编编写从而不会被优化
    __asm__ volatile(
        // 先来个循环（总共是两层循环，内层循环次数 8000000）
        "mov r2, %0\n"
        "mov r3, %1\n"
        "mov r0, #0\n"
        "1:\n"
        // 内层循环
        "mov r1, #0\n"
        "2:\n"
        "add r1, r1, #1\n"
        "cmp r1, r3\n"
        "blt 2b\n"
        // 外层循环
        "add r0, r0, #1\n"
        "cmp r0, r2\n"
        "blt 1b\n"
        :
        : "r"(time), "r"(8000000 / 8)
        : "r0", "r1", "r2", "r3");
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

OS_MEM *CommTxBuf;
INT8U CommTxPart[100][32];

int main()
{
    // 时钟初始化
    RCC_Configuration();
    // 启动systick中断
    OS_CPU_SysTickInitFreq(84000000); // 84Mhz
    my_nvic_priorityGroupConfig(4);
    // LED2初始化
    LED_Init();
    INT8U err = 0;
    CommTxBuf = OSMemCreate(CommTxPart, 100, 32, &err);
    // opt是可选项，根据被置位的位来进行一些额外的操作，暂时没有用到
    (void)OSTaskCreate(test_task, (void *)0, &my_task_0[TASK_STACK_SIZE - 1u], 10);
    (void)OSTaskCreate(task1, (void *)0, &my_task_1[TASK_STACK_SIZE - 1u], 11);
    (void)OSTaskCreate(task2, (void *)0, &my_task_2[TASK_STACK_SIZE - 1u], 12);
    (void)OSTaskCreate(task3, (void *)0, &my_task_3[TASK_STACK_SIZE - 1u], 13);
    (void)OSTaskCreate(task4, (void *)0, &my_task_4[TASK_STACK_SIZE - 1u], 14);
    (void)OSTaskCreate(task5, (void *)0, &my_task_5[TASK_STACK_SIZE - 1u], 15);
    (void)OSTaskCreate(task6, (void *)0, &my_task_6[TASK_STACK_SIZE - 1u], 16);
    (void)OSTaskCreate(task7, (void *)0, &my_task_7[TASK_STACK_SIZE - 1u], 17);
    (void)OSTaskCreate(task8, (void *)0, &my_task_8[TASK_STACK_SIZE - 1u], 18);
    (void)OSTaskCreate(task9, (void *)0, &my_task_9[TASK_STACK_SIZE - 1u], 19);
    (void)OSTaskCreate(task10, (void *)0, &my_task_10[TASK_STACK_SIZE - 1u], 20);
    (void)OSTaskCreate(task11, (void *)0, &my_task_11[TASK_STACK_SIZE - 1u], 21);
    (void)OSTaskCreate(task12, (void *)0, &my_task_12[TASK_STACK_SIZE - 1u], 22);
    (void)OSTaskCreate(task13, (void *)0, &my_task_13[TASK_STACK_SIZE - 1u], 23);
    (void)OSTaskCreate(task14, (void *)0, &my_task_14[TASK_STACK_SIZE - 1u], 24);
    (void)OSTaskCreate(task15, (void *)0, &my_task_15[TASK_STACK_SIZE - 1u], 25);
    (void)OSTaskCreate(task16, (void *)0, &my_task_16[TASK_STACK_SIZE - 1u], 26);
    (void)OSTaskCreate(task17, (void *)0, &my_task_17[TASK_STACK_SIZE - 1u], 27);
    (void)OSTaskCreate(task18, (void *)0, &my_task_18[TASK_STACK_SIZE - 1u], 28);
    (void)OSTaskCreate(task19, (void *)0, &my_task_19[TASK_STACK_SIZE - 1u], 29);
    (void)OSTaskCreate(task20, (void *)0, &my_task_20[TASK_STACK_SIZE - 1u], 30);
    // OS启动
    OSStart();
    return 0;

}

// OS_TICKS_PER_SEC被设置为1000，因此每一个tick为1ms
// 主要测试任务，在空间利用率测试中与其他任务无异
void test_task(void *args)
{
    while (1)
    {
        // led on
        LED_ON();
        // 延时5ms
        OSTimeDly(5);
        // led off
        LED_OFF();
        OSTimeDly(5);
    }
}

// 用于模拟多任务执行环境，并且增加对比度
void task1(void *args)
{
    while (1)
    {
        delay(BLOCK_TIME);
        OSTimeDly(5);
        delay(BLOCK_TIME);
        OSTimeDly(5);
    }
}

// 用于模拟多任务执行环境，并且增加对比度
void task2(void *args)
{
    while (1)
    {
        delay(BLOCK_TIME);
        OSTimeDly(5);
        delay(BLOCK_TIME);
        OSTimeDly(5);
    }
}

// 用于模拟多任务执行环境，并且增加对比度
void task3(void *args)
{
    while (1)
    {
        delay(BLOCK_TIME);
        OSTimeDly(5);
        delay(BLOCK_TIME);
        OSTimeDly(5);
    }
}

// 用于模拟多任务执行环境，并且增加对比度
void task4(void *args)
{
    while (1)
    {
        delay(BLOCK_TIME);
        OSTimeDly(5);
        delay(BLOCK_TIME);
        OSTimeDly(5);
    }
}

// 用于模拟多任务执行环境，并且增加对比度
void task5(void *args)
{
    while (1)
    {
        delay(BLOCK_TIME);
        OSTimeDly(5);
        delay(BLOCK_TIME);
        OSTimeDly(5);
    }
}

// 用于模拟多任务执行环境，并且增加对比度
void task6(void *args)
{
    while (1)
    {
        delay(BLOCK_TIME);
        OSTimeDly(5);
        delay(BLOCK_TIME);
        OSTimeDly(5);
    }
}

// 用于模拟多任务执行环境，并且增加对比度
void task7(void *args)
{
    while (1)
    {
        delay(BLOCK_TIME);
        OSTimeDly(5);
        delay(BLOCK_TIME);
        OSTimeDly(5);
    }
}

// 用于模拟多任务执行环境，并且增加对比度
void task8(void *args)
{
    while (1)
    {
        delay(BLOCK_TIME);
        OSTimeDly(5);
        delay(BLOCK_TIME);
        OSTimeDly(5);
    }
}

// 用于模拟多任务执行环境，并且增加对比度
void task9(void *args)
{
    while (1)
    {
        delay(BLOCK_TIME);
        OSTimeDly(5);
        delay(BLOCK_TIME);
        OSTimeDly(5);
    }
}
// 用于模拟多任务执行环境，并且增加对比度
void task10(void *args)
{
    while (1)
    {
        delay(BLOCK_TIME);
        OSTimeDly(5);
        delay(BLOCK_TIME);
        OSTimeDly(5);
    }
}
// 用于模拟多任务执行环境，并且增加对比度
void task11(void *args)
{
    while (1)
    {
        delay(BLOCK_TIME);
        OSTimeDly(5);
        delay(BLOCK_TIME);
        OSTimeDly(5);
    }
}
// 用于模拟多任务执行环境，并且增加对比度
void task12(void *args)
{
    while (1)
    {
        delay(BLOCK_TIME);
        OSTimeDly(5);
        delay(BLOCK_TIME);
        OSTimeDly(5);
    }
}
// 用于模拟多任务执行环境，并且增加对比度
void task13(void *args)
{
    while (1)
    {
        delay(BLOCK_TIME);
        OSTimeDly(5);
        delay(BLOCK_TIME);
        OSTimeDly(5);
    }
}
// 用于模拟多任务执行环境，并且增加对比度
void task14(void *args)
{
    while (1)
    {
        delay(BLOCK_TIME);
        OSTimeDly(5);
        delay(BLOCK_TIME);
        OSTimeDly(5);
    }
}
// 用于模拟多任务执行环境，并且增加对比度
void task15(void *args)
{
    while (1)
    {
        delay(BLOCK_TIME);
        OSTimeDly(5);
        delay(BLOCK_TIME);
        OSTimeDly(5);
    }
}
// 用于模拟多任务执行环境，并且增加对比度
void task16(void *args)
{
    while (1)
    {
        delay(BLOCK_TIME);
        OSTimeDly(5);
        delay(BLOCK_TIME);
        OSTimeDly(5);
    }
}
// 用于模拟多任务执行环境，并且增加对比度
void task17(void *args)
{
    while (1)
    {
        delay(BLOCK_TIME);
        OSTimeDly(5);
        delay(BLOCK_TIME);
        OSTimeDly(5);
    }
}
// 用于模拟多任务执行环境，并且增加对比度
void task18(void *args)
{
    while (1)
    {
        delay(BLOCK_TIME);
        OSTimeDly(5);
        delay(BLOCK_TIME);
        OSTimeDly(5);
    }
}
// 用于模拟多任务执行环境，并且增加对比度
void task19(void *args)
{
    while (1)
    {
        delay(BLOCK_TIME);
        OSTimeDly(5);
        delay(BLOCK_TIME);
        OSTimeDly(5);
    }
}
// 用于模拟多任务执行环境，并且增加对比度
void task20(void *args)
{
    while (1)
    {
        delay(BLOCK_TIME);
        OSTimeDly(5);
        delay(BLOCK_TIME);
        OSTimeDly(5);
    }
}