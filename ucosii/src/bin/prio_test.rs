#![no_main]
#![no_std]
#![feature(impl_trait_in_assoc_type)]

use defmt::info;
// <- derive attribute
use ucosii::os_core::{OSInit, OSStart};
use ucosii::os_task::{AsyncOSTaskCreate, SyncOSTaskCreate};
use ucosii::os_time::blockdelay::delay;
use ucosii::os_time::timer::Timer;
use ucosii::{self as _};

const LONG_TIME: usize = 10;
const MID_TIME: usize = 5;
const SHORT_TIME: usize = 3;

// fn hello() {
//     defmt::info!("Hello, world!");
// }

#[cortex_m_rt::entry]
fn main_test() -> ! {
    loop {
        test_basic_schedule();
    }
}
fn test_basic_schedule() {
    info!("==========test begin==========");
    // os初始化
    OSInit();
    // 创建6个任务，测试优先级调度的顺序是否正确
    // 调度顺序应该为：task5->task1(task5中创建)->task4->task3->task2->task1->task1(在task4中创建)->task6(由于优先级相同输出相关信息)
    SyncOSTaskCreate(task1, 0 as *mut (), 0 as *mut usize, 30);
    SyncOSTaskCreate(task2, 0 as *mut (), 0 as *mut usize, 25);
    AsyncOSTaskCreate(task3, 0 as *mut (), 0 as *mut usize, 20);
    SyncOSTaskCreate(task4, 0 as *mut (), 0 as *mut usize, 15);
    SyncOSTaskCreate(task5, 0 as *mut (), 0 as *mut usize, 10);
    SyncOSTaskCreate(task6, 0 as *mut (), 0 as *mut usize, 35);
    // 启动os
    OSStart();
}

fn task1(_args: *mut ()) {
    // 任务1
    info!("---task1 begin---");
    delay(LONG_TIME);
    info!("---task1 end---");
    delay(SHORT_TIME);
}
fn task2(_args: *mut ()) {
    // 任务2
    info!("---task2 begin---");
    delay(MID_TIME);
    info!("---task2 end---");
    delay(SHORT_TIME);
}
async fn task3(_args: *mut ()) {
    // 任务3
    //
    info!("---task3 begin---");
    Timer::after_ticks(LONG_TIME as u64).await;
    // delay(LONG_TIME);
    info!("---task3 end---");
    delay(SHORT_TIME);
}
fn task4(_args: *mut ()) {
    // 任务4
    info!("---task4 begin---");
    // 任务4中涉及任务创建
    SyncOSTaskCreate(task1, 0 as *mut (), 0 as *mut usize, 34);
    delay(SHORT_TIME);
    info!("---task4 end---");
    delay(SHORT_TIME);
}

fn task5(_args: *mut ()) {
    // 任务5
    info!("---task5 begin---");
    // 任务5中涉及任务创建
    SyncOSTaskCreate(task1, 0 as *mut (), 0 as *mut usize, 11);
    delay(SHORT_TIME);
    info!("---task5 end---");
    delay(SHORT_TIME);
}

/* 任务6用于测试优先级相同的情况 */
fn task6(_args: *mut ()) {
    // 任务6
    info!("---task6 begin---");
    // 任务6中涉及任务创建，新创建的优先级与当前任务相同
    SyncOSTaskCreate(task1, 0 as *mut (), 0 as *mut usize, 35);
    delay(SHORT_TIME);
    info!("---task6 end---");
    delay(SHORT_TIME);
}
