#ifndef OS_STK_H
#define OS_STK_H
#include "os_cpu.h"
#include "ucos_ii.h"
#define MY_TASK_SIZE_0 256
#define MY_TASK_SIZE_1 256

#define MY_TASK_SIZE_2 1024
OS_STK my_task_0[MY_TASK_SIZE_0];
OS_STK my_task_1[MY_TASK_SIZE_1];
OS_STK my_task_2[MY_TASK_SIZE_2];

OS_EVENT *bottom_sem;                //声明串口发送数据的互斥信号量（信号量本质上是一个事件）

#endif