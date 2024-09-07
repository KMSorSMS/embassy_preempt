#include "tools.h"
// 不能再次include OS_stk，否则会导致OS_stk中的变量重复定义
// #include "OS_stk.h"
#include "stm32f401xe.h"

// 点亮LED2，对应PA5，操作GPIO口
void LED_Init(void)
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

// PC13 在exti13，对应中断号40
void Bottom_Init(){
    /* GPIO端口设置 */
    // 使能GPIOC时钟
    RCC->AHB1ENR |= 0x04;
    // 先设置GPIO的模式，对应GPIO的MODER寄存器
    GPIOC->MODER &= ~0x0C000000; // 先清零，输入模式对应的即为00
    // 设置GPIO的上下拉电阻，对应GPIO的PUPDR寄存器，这里我们设置为下拉
    GPIOC->PUPDR &= ~0x0C000000; // 先清零
    GPIOC->PUPDR |= 0x08000000; // 设置为10，即下拉

    /* EXTI13配置 */
    // 使能SYSCFG时钟
    RCC->APB2ENR |= 0x00004000;
    // 设置PC13为EXTI13的中断输入
    SYSCFG->EXTICR[3] |= 0x0020;
    // 设置EXTI13的中断线为上升沿触发
    EXTI->RTSR |= 0x2000;
    // 使能EXTI13
    EXTI->IMR |= 0x2000;
    // unpend EXTI13, 写1清除
    EXTI->PR = 0x2000;

    /* NVIC配置 */
    // 使能EXTI15_10_IRQn中断
    // 是NVIC_ISER1 的第八位
    NVIC->ISER[1] |= 0x00000100;
    // unpend EXTI15_10_IRQn, 写1清除
    NVIC->ICPR[1] = 0x00000100;
}

// PA0：process pin     PA1：interrupt pin
void Pin_Init(){
    // 先设置GPIO的模式，对应GPIO的MODER寄存器
    GPIOA->MODER &= ~0x0000000F; // 先清零
    GPIOA->MODER |= 0x00000005;  // 设置为01，即输出模式
    // 设置GPIO的输出类型，对应GPIO的OTYPER寄存器
    GPIOA->OTYPER &= ~0x00000003; // 先清零,不过PUSH-PULL模式置为就是为0
    // 设置GPIO的输出速度，对应GPIO的OSPEEDR寄存器，这里我们设置为高速吧
    GPIOA->OSPEEDR &= ~0x0000000F; // 先清零
    GPIOA->OSPEEDR |= 0x0000000A;  // 设置为10，即高速
    // 设置GPIO的上下拉电阻，对应GPIO的PUPDR寄存器，这里我们设置为没有上下拉
    GPIOA->PUPDR &= ~0x0000000F; // 先清零
    // 最后设置GPIO的输出值，对应GPIO的ODR寄存器，这里我们设置为低电平
    GPIOA->ODR &= ~0x00000003; // 设置为1，即高电平
}

// EXTI13中断服务程序
void EXTI15_10_IRQHandler(void)
{
    // 判断是否产生了EXTI13中断
    if (EXTI->PR & 0x2000)
    {
        // set the interrup pin high
        interrupt_pin_high();
        // 清除中断标志位
        EXTI->PR = 0x2000;

        // 释放信号量唤醒对应的任务
        OSSemPost(bottom_sem);
    }
}