#![no_main]
#![no_std]
#![feature(impl_trait_in_assoc_type)]

#[cfg(feature = "defmt")]
use defmt::info;
// <- derive attribute
use ucosii::os_core::{OSInit, OSStart};
use ucosii::os_task::{AsyncOSTaskCreate, SyncOSTaskCreate};
use ucosii::os_time::blockdelay::delay;
use ucosii::os_time::timer::Timer;
// use ucosii::{self as _};

const LONG_TIME: usize = 10;
const MID_TIME: usize = 5;
const SHORT_TIME: usize = 3;

// fn hello() {
//     defmt::// #info!("Hello, world!");
// }

#[cortex_m_rt::entry]
fn main_test() -> ! {
    loop {
        test_basic_schedule();
    }
}
fn test_basic_schedule() {
    // os初始化
    OSInit();
    // 创建两个任务
    SyncOSTaskCreate(task1, 0 as *mut (), 0 as *mut usize, 10);
    SyncOSTaskCreate(task2, 0 as *mut (), 0 as *mut usize, 11);
    AsyncOSTaskCreate(task3, 0 as *mut (), 0 as *mut usize, 12);
    SyncOSTaskCreate(task4, 0 as *mut (), 0 as *mut usize, 13);
    // 启动os
    OSStart();
}

fn task1(_args: *mut ()) {
    // 任务1
    // #info!("---task1 begin---");
    delay(LONG_TIME);
    // #info!("---task1 end---");
    delay(SHORT_TIME);
}
fn task2(_args: *mut ()) {
    // 任务2
    // #info!("---task2 begin---");
    delay(MID_TIME);
    // #info!("---task2 end---");
    delay(SHORT_TIME);
}
async fn task3(_args: *mut ()) {
    // 任务3
    loop {
        //
        // #info!("---task3 begin---");
        Timer::after_ticks(LONG_TIME as u64).await;
        // delay(LONG_TIME);
        Timer::after_millis(3000).await;
        // #info!("---task3 end---");
        delay(SHORT_TIME);
    }
}
fn task4(_args: *mut ()) {
    // 任务4
    // #info!("---task4 begin---");
    // 任务3中涉及任务创建
    SyncOSTaskCreate(task1, 0 as *mut (), 0 as *mut usize, 14);
    delay(SHORT_TIME);
    // #info!("---task4 end---");
    delay(MID_TIME);
}
