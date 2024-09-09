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

void test_task(void *args)
{
    INT8U err=0;
    while (1)
    {
        thread_pin_low();
        OSSemPend(bottom_sem, 0, &err);
        interrupt_pin_low();
        thread_pin_high();
        OSTimeDly(5); //delay 5 ms
        thread_pin_low();
        OSSemPend(bottom_sem, 0, &err);
        interrupt_pin_low();
        thread_pin_high();
        OSTimeDly(5); //delay 5 ms
    }
}

void task1(void *args)
{
    while (1)
    {
        LED_ON()
        OSTimeDly(5* 100); //delay 5 ms
        LED_OFF()
        OSTimeDly(5* 100); //delay 5 ms
    }
}

void task2(void *args)
{
    while (1)
    {
        delay(BLOCK_TIME);
        OSTimeDly(5); //delay 5 ms
        delay(BLOCK_TIME);
        OSTimeDly(5); //delay 5 ms
    }
}

void task3(void *args)
{
    while (1)
    {
        delay(BLOCK_TIME);
        OSTimeDly(5); //delay 5 ms
        delay(BLOCK_TIME);
        OSTimeDly(5); //delay 5 ms
    }
}
void task4(void *args)
{
    while (1)
    {
        delay(BLOCK_TIME);
        OSTimeDly(5); //delay 5 ms
        delay(BLOCK_TIME);
        OSTimeDly(5); //delay 5 ms
    }
}

void task5(void *args)
{
    while (1)
    {
        delay(BLOCK_TIME);
        OSTimeDly(5); //delay 5 ms
        delay(BLOCK_TIME);
        OSTimeDly(5); //delay 5 ms
    }
}


int main(){
    // 时钟初始化
    RCC_Configuration();
    // 启动systick中断
    OS_CPU_SysTickInitFreq(84000000); // 84Mhz
    my_nvic_priorityGroupConfig(4);
    // LED2初始化
    LED_Init();
    // 初始化thread pin以及interrupt pin
    Pin_Init();
    // 初始化按键以及按键中断
    Bottom_Init();
    OSInit();
    // 创建按键中断信号量(需要注意的是信号量的创建只能在OSInit之后，因为需要使用OSInit中的事件队列)
    bottom_sem=OSSemCreate(0);
    // OS_TRACE_START();
    // 创建两个个自己的任务
    // pnext用于指向任务扩展部分，暂时没有用到
    // opt是可选项，根据被置位的位来进行一些额外的操作，暂时没有用到
    (void)OSTaskCreate(test_task, (void *)0, &my_task_0[TASK_STACK_SIZE - 1u], 15);
    (void)OSTaskCreate(task1, (void *)0, &my_task_1[TASK_STACK_SIZE - 1u], 16);
    (void)OSTaskCreate(task2, (void *)0, &my_task_2[TASK_STACK_SIZE - 1u], 17);
    (void)OSTaskCreate(task3, (void *)0, &my_task_3[TASK_STACK_SIZE- 1u], 18);
    (void)OSTaskCreate(task4, (void *)0, &my_task_4[TASK_STACK_SIZE- 1u], 19);
    (void)OSTaskCreate(task5, (void *)0, &my_task_5[TASK_STACK_SIZE- 1u], 20);

    // OS启动
    OSStart();
    return 0;
}