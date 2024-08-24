#include "stm32f401xe.h"
#include <stdint.h>

// 定义OS_STK
typedef uint32_t OS_STK;
// 声明Rust函数
extern uint8_t OSTaskCreate(void (*fun_ptr)(void *), void *p_arg, OS_STK *ptos,
                             uint8_t prio);
extern void OSInit();
extern void OSStart();
// 开灯操作，用宏定义
#define LED2_ON() GPIOA->ODR |= 0x00000020; // 设置为1，即高电平
// 关灯操作，用宏定义
#define LED2_OFF() GPIOA->ODR &= ~0x00000020; // 设置为0，即低电平

// 阻塞函数
void delay(uint32_t time) {
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
// 定义任务函数
void task(void *p_arg) {
  while (1) {
    LED2_ON();
    delay(5);
    LED2_OFF();
    delay(5);
  }
}

// 点亮LED2，对应PA5，操作GPIO口
void LED2_Init(void) {
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
// 写一个时钟初始化的函数，配置为HSE，PLLM为4，PLLN为84，PLLP分频为2，PLLQ分频为4，还有AHB的地方分频为1
// ，得到主频为84Mhz
void RCC_Configuration(void) {
  // 先把PLL和PLL2S disable了
  RCC->CR &= ~0x05000000;
  // 先该各个分频系数,并且选择PLLSRC，把HSE设置为PLL的输入源
  RCC->PLLCFGR =
      0b00000100010000000001010100000100; // 0x4401504的二进制是：0000 0100 0100
                                          // 0000 0001 0101 0000 0100
  // 上面的配置是：PLLM=4, PLLN=84, PLLP=2, PLLQ=4，并且设置HSE为PLL的输入源
  // 设置AHB的分频系数为1
  RCC->CFGR &= ~0xF0;
  // 设置APB1的分频系数为2，APB2的分频系数为1
  RCC->CFGR |= 0x1000;
  RCC->CFGR &= ~0xE000;
  // 设置启动HSE，开启PLL和PLL2S
  RCC->CR |= 0b00000101000000010000000000000000; // 0x5010000
  // 加入保护代码，检查HSE和PLL、PLL2S的启动状态：
  while ((RCC->CR & 0x00020000) == 0)
    ; // 等待HSE启动成功
  while ((RCC->CR & 0x02000000) == 0)
    ; // 等待PLL启动成功
  while ((RCC->CR & 0x08000000) == 0)
    ; // 等待PLL2S启动成功
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
OS_STK my_task_0[1024];

int main() {
  // 时钟初始化
  RCC_Configuration();
  LED2_Init();
  LED2_OFF();
  OSInit();
  // 调用Rust函数
  OSTaskCreate(task, 0, &my_task_0[1024 - 1u], 12);
  OSStart();
  while (1)
    ;
}
