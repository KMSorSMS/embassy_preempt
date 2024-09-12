#ifndef OS_STK_H
#define OS_STK_H
#include "os_cpu.h"
#include "ucos_ii.h"
// 任务栈的大小设置为同一个值（因为任务的内容几乎没有区别）
#define TASK_STACK_SIZE 18
OS_STK my_task_0[TASK_STACK_SIZE];
OS_STK my_task_1[TASK_STACK_SIZE];
OS_STK my_task_2[TASK_STACK_SIZE];
OS_STK my_task_3[TASK_STACK_SIZE];
OS_STK my_task_4[TASK_STACK_SIZE];
OS_STK my_task_5[TASK_STACK_SIZE];
OS_STK my_task_6[TASK_STACK_SIZE];
OS_STK my_task_7[TASK_STACK_SIZE];
OS_STK my_task_8[TASK_STACK_SIZE];
OS_STK my_task_9[TASK_STACK_SIZE];
OS_STK my_task_10[TASK_STACK_SIZE];
OS_STK my_task_11[TASK_STACK_SIZE];
OS_STK my_task_12[TASK_STACK_SIZE];
OS_STK my_task_13[TASK_STACK_SIZE];
OS_STK my_task_14[TASK_STACK_SIZE];
OS_STK my_task_15[TASK_STACK_SIZE];
OS_STK my_task_16[TASK_STACK_SIZE];
OS_STK my_task_17[TASK_STACK_SIZE];
OS_STK my_task_18[TASK_STACK_SIZE];
OS_STK my_task_19[TASK_STACK_SIZE];
OS_STK my_task_20[TASK_STACK_SIZE];

OS_EVENT *bottom_sem;                //声明串口发送数据的互斥信号量（信号量本质上是一个事件）

#endif