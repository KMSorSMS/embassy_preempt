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

#define USART_RX_TASK_SIZE 256
OS_STK uasrt_rx_task[USART_RX_TASK_SIZE];

OS_EVENT *uasrt_tx_sem;                //声明串口发送数据的互斥信号量（信号量本质上是一个事件）
OS_EVENT *uasrt_rx_sem;                //声明串口接收数据的资源信号量（这个资源最多只有一条所以可以直接当互斥信号量用）
// OS_EVENT *uasrt_rx_sem;
#define MOTOR_TASK_STK_SIZE 1024
OS_STK motor_change_task_stk[MOTOR_TASK_STK_SIZE];
#endif
