#ifndef TOOLS_H
#define TOOLS_H


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

// 先是初始化函数
void LED_Init(void);
void Pin_Init(void);
void Bottom_Init(void);

#endif