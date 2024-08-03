#![no_std]
#![no_main]

use core::arch::asm;

use defmt::println;
use ucosii::os_task::OSTaskCreate;
use ucosii::os_time::Timer;
use ucosii::{self as _};
// See https://crates.io/crates/defmt-test/0.3.0 for more documentation (e.g. about the 'state'
// feature)
const LONG_TIME: usize = 10;
const MID_TIME: usize = 5;
const SHORT_TIME: usize = 3;
#[defmt_test::tests]
mod tests {
    use ucosii::os_core::{OSInit, OSStart};
    use ucosii::os_task::{OSTaskCreate, RustOSTaskCreate};

    use crate::{task1, task2, task3, task4};

    #[test]
    fn test_basic_schedule() {
        // os初始化
        OSInit();
        // 创建两个任务
        OSTaskCreate(task1, 0 as *mut (), 0 as *mut usize, 10);
        OSTaskCreate(task2, 0 as *mut (), 0 as *mut usize, 11);
        RustOSTaskCreate(task3, 0 as *mut (), 0 as *mut usize, 12);
        OSTaskCreate(task4, 0 as *mut (), 0 as *mut usize, 13);
        // 启动os
        OSStart();
    }
}
fn task1(_args: *mut ()) {
    // 任务1
    println!("---task1 begin---");
    delay(LONG_TIME);
    println!("---task1 end---");
    delay(SHORT_TIME);
}
fn task2(_args: *mut ()) {
    // 任务2
    println!("---task2 begin---");
    delay(MID_TIME);
    println!("---task2 end---");
    delay(SHORT_TIME);
}
async fn task3(_args: *mut ()) {
    // 任务3
    println!("---task3 begin---");
    delay(LONG_TIME);
    println!("---task3 end---");
    delay(SHORT_TIME);
}
fn task4(_args: *mut ()) {
    // 任务4
    println!("---task4 begin---");
    // 任务4中涉及任务创建
    OSTaskCreate(task1, 0 as *mut (), 0 as *mut usize, 14);
    delay(SHORT_TIME);
    println!("---task4 end---");
    delay(SHORT_TIME);
}
#[inline]
fn delay(time: usize) {
    // 延时函数,time的单位约为0.5s，使用汇编编写从而不会被优化
    unsafe {
        asm!(
            // 先来个循环（总共是两层循环，内层循环次数8000000）
            "mov r0, #0",
            "1:",
            // 内层循环
            "mov r1, #0",
            "2:",
            "add r1, r1, #1",
            "cmp r1, r3",
            "blt 2b",
            // 外层循环
            "add r0, r0, #1",
            "cmp r0, r2",
            "blt 1b",
            in("r2") time,
            in("r3") 8000000/8,
        )
    }
}
