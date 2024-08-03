#![no_std]
#![no_main]

use core::arch::asm;

use defmt::println;
use ucosii::{self as _, os_task::OSTaskCreate, os_time::Timer};
// See https://crates.io/crates/defmt-test/0.3.0 for more documentation (e.g. about the 'state'
// feature)
const LONG_TIME: usize = 10;
const MID_TIME: usize = 5;
const SHORT_TIME: usize = 3;
#[defmt_test::tests]
mod tests {
    use ucosii::{os_core::{OSInit, OSStart}, os_task::{OSTaskCreate, RustOSTaskCreate}};

    use crate::{task1,task2,task3, task4};

    #[test]
    fn test_basic_schedule() {
        // os初始化
        OSInit();
        // 创建两个任务
        OSTaskCreate(task1, 0 as *mut (), 0 as *mut usize, 10);
        OSTaskCreate(task2, 0 as *mut (), 0 as *mut usize, 11);
        RustOSTaskCreate(task3, 0 as *mut (),0 as *mut usize, 12);
        OSTaskCreate(task4, 0 as *mut (), 0 as *mut usize, 13);
        // 启动os
        OSStart();
    }
}
fn task1(_args:*mut ()) {
    // 任务1
    println!("---task1 begin---");
    delay(LONG_TIME);
    println!("---task1 end---");
    delay(SHORT_TIME);
}
fn task2(_args:*mut ()) {
    // 任务2
    println!("---task2 begin---");
    delay(MID_TIME);
    println!("---task2 end---");
    delay(SHORT_TIME);
}
async fn task3(_args:*mut ()) {
    // 任务3
    println!("---task3 begin---");
    delay(LONG_TIME);
    println!("---task3 end---");
    delay(SHORT_TIME);
}
fn task4(_args:*mut ()) {
    // 任务4
    println!("---task4 begin---");
    // 任务4中涉及任务创建
    OSTaskCreate(task1, 0 as *mut (), 0 as *mut usize, 14);
    delay(SHORT_TIME);
    println!("---task4 end---");
    delay(SHORT_TIME);
}
fn delay(time: usize){
    // 延时函数,time的单位约为0.5s
    for _ in 0..time {
        // 记得改成/2是较慢的（后面有抢占的时候需要）
        for _ in 0..200000/8 {
            unsafe {
                asm!("nop");
            }
        }
    }
}