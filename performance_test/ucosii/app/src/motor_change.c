#include "motor_change.h"
#include "Reciever.h"
#include "Motor.h"

void changeMotorTask(void *args){
	(void)args;
	while(1){
		print_capture_pwm((uint16_t)1);
        OSTimeDly(1000 * 3); // 延时5s，因为一个tick是1毫秒
	}
}