#include "stm32f4xx_it.h"
#include "ucos_ii.h"

void SysTick_Handler(void) { OS_CPU_SysTickHandler(); }
