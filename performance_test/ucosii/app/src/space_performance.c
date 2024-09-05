#include "main.h"
#include "NVIC.h"
#include "OS_stk.h"
#include "os_cpu.h"
#include "tools.h"
#include "ucos_ii.h"
#include "os_trace.h"

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
INT8U   CommTxPart[100][32];

int main(){
    // 时钟初始化
    RCC_Configuration();
    // 启动systick中断
    OS_CPU_SysTickInitFreq(84000000); // 84Mhz
    my_nvic_priorityGroupConfig(4);
    // LED2初始化
    LED_Init();
    INT8U err=0;
    CommTxBuf = OSMemCreate(CommTxPart, 100, 32, &err);
    // opt是可选项，根据被置位的位来进行一些额外的操作，暂时没有用到
    (void)OSTaskCreate(test_bottom, (void *)0, &my_task_0[MY_TASK_SIZE_0 - 1u], 40);
    (void)OSTaskCreate(my_task_1_t_, (void *)0, &my_task_1[MY_TASK_SIZE_1 - 1u], 39);
    //串口接收数据任务（发送不创建任务，而是直接使用线程安全的print）
    // OS启动
    OSStart();
    return 0;
}

void test_bottom(void *args)
{
    while (1)
    {
        LED_OFF()
        // delay_used_by_iic(100*1000*3);
        // LED2_ON();
        // delay_used_by_iic(100*1000*3);
        // 任务0是关灯，关完后调用OS延时函数--OSTimeDly()
        OSTimeDly(1000 * 4); // 延时10s，因为一个tick是10微秒
    }
}

void my_task_1_t_(void *args)
{
    while (1)
    {
        // 任务一采用点灯，点完后调用OS延时函数--OSTimeDly()
        LED_ON()
        OSTimeDly(1000 * 2); // 延时5s，因为一个tick是10微秒
    }
}