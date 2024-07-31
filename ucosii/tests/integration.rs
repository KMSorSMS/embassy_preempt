#![no_std]
#![no_main]

use core::arch::asm;

use defmt::println;
use ucosii::{self as _, os_task::OSTaskCreate};
// See https://crates.io/crates/defmt-test/0.3.0 for more documentation (e.g. about the 'state'
// feature)
const LONG_TIME: usize = 10;
const MID_TIME: usize = 5;
const SHORT_TIME: usize = 2;
#[defmt_test::tests]
mod tests {
    use defmt::assert;
    use ucosii::{os_core::{OSInit, OSStart}, os_task::{OSTaskCreate, RustOSTaskCreate}};

    use crate::{task1,task2,task3, task4};

    #[test]
    fn it_works() {
        assert!(true)
    }
    // #[test]
    // fn test_os_init() {
    //     // os初始化
    //     OSInit();
    //     // 创建两个任务
    //     OSTaskCreate(task1, 0 as *mut (), 0 as *mut usize, 10);
    //     OSTaskCreate(task2, 0 as *mut (), 0 as *mut usize, 11);
    //     RustOSTaskCreate(task3, 0 as *mut (),0 as *mut usize, 12);
    //     OSTaskCreate(task4, 0 as *mut (), 0 as *mut usize, 13);
    //     // 启动os
    //     OSStart();
    // }
}
fn task1(_args:*mut ()) {
    // 任务1
    println!("---task1 begin---");
    delay(LONG_TIME);
    println!("---task1 end---");
}
fn task2(_args:*mut ()) {
    // 任务2
    println!("---task2 begin---");
    delay(MID_TIME);
    println!("---task2 end---");
}
async fn task3(_args:*mut ()) {
    // 任务3
    println!("---task3 begin---");
    delay(LONG_TIME);
    println!("---task3 end---");
}
fn task4(_args:*mut ()) {
    // 任务4
    println!("---task4 begin---");
    // 任务3中涉及任务创建
    OSTaskCreate(task1, 0 as *mut (), 0 as *mut usize, 14);
    delay(SHORT_TIME);
    println!("---task4 end---");
}
fn delay(time: usize){
    // 延时函数,time的单位约为1s
    for _ in 0..time {
        for _ in 0..8000000*2 {
            unsafe {
                asm!("nop");
            }
        }
    }
}