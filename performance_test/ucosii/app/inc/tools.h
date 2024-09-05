#ifndef TOOLS_H
#define TOOLS_H
#include "ucos_ii.h"

extern OS_EVENT *bottom_sem;

// 开灯操作，用宏定义
#define LED_ON() GPIOA->ODR |= 0x00000020; // 设置为1，即高电平
// 关灯操作，用宏定义
#define LED_OFF() GPIOA->ODR &= ~0x00000020; // 设置为0，即低电平

// 引脚电平设置函数
// thread pin is PA0
#define thread_pin_high() GPIOA->ODR |= 0x00000001; // 设置为1，即高电平
#define thread_pin_low() GPIOA->ODR &= ~0x00000001; // 设置为0，即低电平
// interrupt pin is PA1 interrupt_pin_low
#define interrupt_pin_high() GPIOA->ODR |= 0x00000002; // 设置为1，即高电平
#define interrupt_pin_low() GPIOA->ODR &= ~0x00000002; // 设置为0，即低电平

// 自己实现的阻塞延时函数，延时精度10微秒
// 利用 OSTime 变量，我们通过预先设定了OS_TICKS_PER_SEC为100000保证了10微秒一次的中断，OSTime10微秒加一次，这里采取阻塞等待
// 追加static inline因为这个函数内容少。
static inline void Delay_Congestion(INT32U delay_10us)
{
    // 当前的OStime
    INT32U cur_time = OSTime;
    // 阻塞等待，注意OSTime可能会溢出，溢出方式是从0开始，所以需要对这个情况进行处理,这里不用加锁，因为只是查看OStime
    // while ( (OSTime < cur_time && (0xFFFFFFFF - cur_time) + OSTime < delay_10us ) || OSTime - cur_time < delay_10us );
    // 突然想到，无符号数如果不够减，会自动借位，所以可以简化为
    while (OSTime - cur_time < delay_10us);
}

// 先是初始化函数
void LED_Init(void);
void Pin_Init(void);
void Bottom_Init(void);

#endif