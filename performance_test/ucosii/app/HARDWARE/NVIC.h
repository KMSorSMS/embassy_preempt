#pragma once
#ifndef __NVIC_H
#define __NVIC_H
#include "main.h"
uint32_t get_nvic_group(void);
/**
* 设分组
*/
void my_nvic_priorityGroupConfig(uint32_t NVIC_PriorityGroup);
/**
* 设置优先级
* 中断号，抢占优先级，响应优先级
*/
void my_nvic_set_priority(uint8_t NVIC_IRQChannel,uint8_t NVIC_IRQChannelPreemptionPriority,uint8_t NVIC_IRQChannelSubPriority);
void my_nvic_enable(uint8_t NVIC_IRQChannel);

#endif
