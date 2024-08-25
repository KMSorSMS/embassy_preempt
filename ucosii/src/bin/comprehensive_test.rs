#![no_main]
#![no_std]
#![feature(impl_trait_in_assoc_type)]

use core::ffi::c_void;

#[cfg(feature = "defmt")]
use defmt::info;
// <- derive attribute
use ucosii::os_core::{OSInit, OSStart};
use ucosii::os_task::{AsyncOSTaskCreate, SyncOSTaskCreate};
// use ucosii::os_time::blockdelay::delay;
use ucosii::os_time::timer::Timer;
use ucosii::os_time::OSTimeDly;
// use ucosii::{self as _};

// the unit is tick
// test long TIME delay
const LONG_LONG_TIME: u64 = 100000;
const LONG_TIME: u64 = 10000;
// test high frequency interrupt
const MID_TIME: u64 = 100;
const SHORT_TIME: u64 = 10;
// the set alarm will return false
const SHORT_SHORT_TIME: u64 = 1;

#[cortex_m_rt::entry]
fn test_basic_schedule() -> ! {
    // os初始化
    OSInit();
    // TASK create. The prio should be interlude, and the prio should be low to high
    // create ASYNC TASK
    AsyncOSTaskCreate(task1, 0 as *mut c_void, 0 as *mut usize, 38);
    AsyncOSTaskCreate(task2, 0 as *mut c_void, 0 as *mut usize, 36);
    AsyncOSTaskCreate(task3, 0 as *mut c_void, 0 as *mut usize, 34);
    AsyncOSTaskCreate(task4, 0 as *mut c_void, 0 as *mut usize, 32);
    AsyncOSTaskCreate(task5, 0 as *mut c_void, 0 as *mut usize, 30);
    AsyncOSTaskCreate(task6, 0 as *mut c_void, 0 as *mut usize, 28);
    AsyncOSTaskCreate(task7, 0 as *mut c_void, 0 as *mut usize, 26);
    AsyncOSTaskCreate(task8, 0 as *mut c_void, 0 as *mut usize, 24);
    AsyncOSTaskCreate(task9, 0 as *mut c_void, 0 as *mut usize, 22);
    AsyncOSTaskCreate(task10, 0 as *mut c_void, 0 as *mut usize, 20);
    AsyncOSTaskCreate(task11, 0 as *mut c_void, 0 as *mut usize, 18);
    AsyncOSTaskCreate(task12, 0 as *mut c_void, 0 as *mut usize, 16);
    AsyncOSTaskCreate(task13, 0 as *mut c_void, 0 as *mut usize, 14);
    AsyncOSTaskCreate(task14, 0 as *mut c_void, 0 as *mut usize, 12);
    AsyncOSTaskCreate(task15, 0 as *mut c_void, 0 as *mut usize, 10);
    // create SYNC TASK
    SyncOSTaskCreate(task16, 0 as *mut c_void, 0 as *mut usize, 39);
    SyncOSTaskCreate(task17, 0 as *mut c_void, 0 as *mut usize, 37);
    SyncOSTaskCreate(task18, 0 as *mut c_void, 0 as *mut usize, 35);
    SyncOSTaskCreate(task19, 0 as *mut c_void, 0 as *mut usize, 33);
    SyncOSTaskCreate(task20, 0 as *mut c_void, 0 as *mut usize, 31);
    SyncOSTaskCreate(task21, 0 as *mut c_void, 0 as *mut usize, 29);
    SyncOSTaskCreate(task22, 0 as *mut c_void, 0 as *mut usize, 27);
    SyncOSTaskCreate(task23, 0 as *mut c_void, 0 as *mut usize, 25);
    SyncOSTaskCreate(task24, 0 as *mut c_void, 0 as *mut usize, 23);
    SyncOSTaskCreate(task25, 0 as *mut c_void, 0 as *mut usize, 21);
    SyncOSTaskCreate(task26, 0 as *mut c_void, 0 as *mut usize, 19);
    SyncOSTaskCreate(task27, 0 as *mut c_void, 0 as *mut usize, 17);
    SyncOSTaskCreate(task28, 0 as *mut c_void, 0 as *mut usize, 15);
    SyncOSTaskCreate(task29, 0 as *mut c_void, 0 as *mut usize, 13);
    SyncOSTaskCreate(task30, 0 as *mut c_void, 0 as *mut usize, 11);
    // 启动os
    OSStart();
}

// ASYNC TEST
// ss-ss
async fn task1(_args: *mut c_void) {
    loop {
        // 任务1
        #[cfg(feature = "defmt")]
        info!("---task1 begin---");

        Timer::after_ticks(SHORT_SHORT_TIME).await;
    
        #[cfg(feature = "defmt")]
        info!("---task1 end---");
        
        Timer::after_ticks(SHORT_SHORT_TIME).await;
    }
}

// ss-s
async fn task2(_args: *mut c_void) {
    loop {
        // 任务1
        #[cfg(feature = "defmt")]
        info!("---task2 begin---");

        Timer::after_ticks(SHORT_SHORT_TIME).await;
    
        #[cfg(feature = "defmt")]
        info!("---task2 end---");
        
        Timer::after_ticks(SHORT_TIME).await;
    }
}

// ss-m
async fn task3(_args: *mut c_void) {
    loop {
        // 任务1
        #[cfg(feature = "defmt")]
        info!("---task3 begin---");

        Timer::after_ticks(SHORT_SHORT_TIME).await;
    
        #[cfg(feature = "defmt")]
        info!("---task3 end---");
        
        Timer::after_ticks(MID_TIME).await;
    }
}

// ss-l
async fn task4(_args: *mut c_void) {
    loop {
        // 任务1
        #[cfg(feature = "defmt")]
        info!("---task4 begin---");

        Timer::after_ticks(SHORT_SHORT_TIME).await;
    
        #[cfg(feature = "defmt")]
        info!("---task4 end---");
        
        Timer::after_ticks(LONG_TIME).await;
    }
}

// ss-ll
async fn task5(_args: *mut c_void) {
    loop {
        // 任务1
        #[cfg(feature = "defmt")]
        info!("---task5 begin---");

        Timer::after_ticks(SHORT_SHORT_TIME).await;
    
        #[cfg(feature = "defmt")]
        info!("---task5 end---");
        
        Timer::after_ticks(LONG_LONG_TIME).await;
    }
}

// s-s
async fn task6(_args: *mut c_void) {
    loop {
        // 任务1
        #[cfg(feature = "defmt")]
        info!("---task6 begin---");

        Timer::after_ticks(SHORT_TIME).await;
    
        #[cfg(feature = "defmt")]
        info!("---task6 end---");
        
        Timer::after_ticks(SHORT_TIME).await;
    }
}

// s-m
async fn task7(_args: *mut c_void) {
    loop {
        // 任务1
        #[cfg(feature = "defmt")]
        info!("---task7 begin---");

        Timer::after_ticks(SHORT_TIME).await;
    
        #[cfg(feature = "defmt")]
        info!("---task7 end---");
        
        Timer::after_ticks(MID_TIME).await;
    }
}

// s-l
async fn task8(_args: *mut c_void) {
    loop {
        // 任务1
        #[cfg(feature = "defmt")]
        info!("---task8 begin---");

        Timer::after_ticks(SHORT_TIME).await;
    
        #[cfg(feature = "defmt")]
        info!("---task8 end---");
        
        Timer::after_ticks(LONG_TIME).await;
    }
}

// s-ll
async fn task9(_args: *mut c_void) {
    loop {
        // 任务1
        #[cfg(feature = "defmt")]
        info!("---task9 begin---");

        Timer::after_ticks(SHORT_TIME).await;
    
        #[cfg(feature = "defmt")]
        info!("---task9 end---");
        
        Timer::after_ticks(LONG_LONG_TIME).await;
    }
}

// m-m
async fn task10(_args: *mut c_void) {
    loop {
        // 任务1
        #[cfg(feature = "defmt")]
        info!("---task10 begin---");

        Timer::after_ticks(MID_TIME).await;
    
        #[cfg(feature = "defmt")]
        info!("---task10 end---");
        
        Timer::after_ticks(MID_TIME).await;
    }
}

// m-l
async fn task11(_args: *mut c_void) {
    loop {
        // 任务1
        #[cfg(feature = "defmt")]
        info!("---task11 begin---");

        Timer::after_ticks(MID_TIME).await;
    
        #[cfg(feature = "defmt")]
        info!("---task11 end---");
        
        Timer::after_ticks(LONG_TIME).await;
    }
}

// m-ll
async fn task12(_args: *mut c_void) {
    loop {
        // 任务1
        #[cfg(feature = "defmt")]
        info!("---task12 begin---");

        Timer::after_ticks(MID_TIME).await;
    
        #[cfg(feature = "defmt")]
        info!("---task12 end---");
        
        Timer::after_ticks(LONG_LONG_TIME).await;
    }
}

// l-l
async fn task13(_args: *mut c_void) {
    loop {
        // 任务1
        #[cfg(feature = "defmt")]
        info!("---task13 begin---");

        Timer::after_ticks(LONG_TIME).await;
    
        #[cfg(feature = "defmt")]
        info!("---task13 end---");
        
        Timer::after_ticks(LONG_TIME).await;
    }
}

// l-ll
async fn task14(_args: *mut c_void) {
    loop {
        // 任务1
        #[cfg(feature = "defmt")]
        info!("---task14 begin---");

        Timer::after_ticks(LONG_TIME).await;
    
        #[cfg(feature = "defmt")]
        info!("---task14 end---");
        
        Timer::after_ticks(LONG_LONG_TIME).await;
    }
}

// ll-ll
async fn task15(_args: *mut c_void) {
    loop {
        // 任务1
        #[cfg(feature = "defmt")]
        info!("---task15 begin---");

        Timer::after_ticks(LONG_LONG_TIME).await;
    
        #[cfg(feature = "defmt")]
        info!("---task15 end---");
        
        Timer::after_ticks(LONG_LONG_TIME).await;
    }
}

// SYNC TEST
// ss-ss
fn task16(_args: *mut c_void) {
    loop {
        // 任务1
        #[cfg(feature = "defmt")]
        info!("---task16 begin---");

        OSTimeDly(SHORT_SHORT_TIME);
    
        #[cfg(feature = "defmt")]
        info!("---task16 end---");
        
        OSTimeDly(SHORT_SHORT_TIME);
    }
}

// ss-s
fn task17(_args: *mut c_void) {
    loop {
        // 任务1
        #[cfg(feature = "defmt")]
        info!("---task17 begin---");

        OSTimeDly(SHORT_SHORT_TIME);
    
        #[cfg(feature = "defmt")]
        info!("---task17 end---");
        
        OSTimeDly(SHORT_TIME);
    }
}

// ss-m
fn task18(_args: *mut c_void) {
    loop {
        // 任务1
        #[cfg(feature = "defmt")]
        info!("---task18 begin---");

        OSTimeDly(SHORT_SHORT_TIME);
    
        #[cfg(feature = "defmt")]
        info!("---task18 end---");
        
        OSTimeDly(MID_TIME);
    }
}

// ss-l
fn task19(_args: *mut c_void) {
    loop {
        // 任务1
        #[cfg(feature = "defmt")]
        info!("---task19 begin---");

        OSTimeDly(SHORT_SHORT_TIME);
    
        #[cfg(feature = "defmt")]
        info!("---task19 end---");
        
        OSTimeDly(LONG_TIME);
    }
}

// ss-ll
fn task20(_args: *mut c_void) {
    loop {
        // 任务1
        #[cfg(feature = "defmt")]
        info!("---task20 begin---");

        OSTimeDly(SHORT_SHORT_TIME);
    
        #[cfg(feature = "defmt")]
        info!("---task20 end---");
        
        OSTimeDly(LONG_LONG_TIME);
    }
}

// s-s
fn task21(_args: *mut c_void) {
    loop {
        // 任务1
        #[cfg(feature = "defmt")]
        info!("---task21 begin---");

        OSTimeDly(SHORT_TIME);
    
        #[cfg(feature = "defmt")]
        info!("---task21 end---");
        
        OSTimeDly(SHORT_TIME);
    }
}

// s-m
fn task22(_args: *mut c_void) {
    loop {
        // 任务1
        #[cfg(feature = "defmt")]
        info!("---task22 begin---");

        OSTimeDly(SHORT_TIME);
    
        #[cfg(feature = "defmt")]
        info!("---task22 end---");
        
        OSTimeDly(MID_TIME);
    }
}

// s-l
fn task23(_args: *mut c_void) {
    loop {
        // 任务1
        #[cfg(feature = "defmt")]
        info!("---task23 begin---");

        OSTimeDly(SHORT_TIME);
    
        #[cfg(feature = "defmt")]
        info!("---task23 end---");
        
        OSTimeDly(LONG_TIME);
    }
}

// s-ll
fn task24(_args: *mut c_void) {
    loop {
        // 任务1
        #[cfg(feature = "defmt")]
        info!("---task24 begin---");

        OSTimeDly(SHORT_TIME);
    
        #[cfg(feature = "defmt")]
        info!("---task24 end---");
        
        OSTimeDly(LONG_LONG_TIME);
    }
}

// m-m
fn task25(_args: *mut c_void) {
    loop {
        // 任务1
        #[cfg(feature = "defmt")]
        info!("---task25 begin---");

        OSTimeDly(MID_TIME);
    
        #[cfg(feature = "defmt")]
        info!("---task25 end---");
        
        OSTimeDly(MID_TIME);
    }
}

// m-l
fn task26(_args: *mut c_void) {
    loop {
        // 任务1
        #[cfg(feature = "defmt")]
        info!("---task26 begin---");

        OSTimeDly(MID_TIME);
    
        #[cfg(feature = "defmt")]
        info!("---task26 end---");
        
        OSTimeDly(LONG_TIME);
    }
}

// m-ll
fn task27(_args: *mut c_void) {
    loop {
        // 任务1
        #[cfg(feature = "defmt")]
        info!("---task27 begin---");

        OSTimeDly(MID_TIME);
    
        #[cfg(feature = "defmt")]
        info!("---task27 end---");
        
        OSTimeDly(LONG_LONG_TIME);
    }
}

// l-l
fn task28(_args: *mut c_void) {
    loop {
        // 任务1
        #[cfg(feature = "defmt")]
        info!("---task28 begin---");

        OSTimeDly(LONG_TIME);
    
        #[cfg(feature = "defmt")]
        info!("---task28 end---");
        
        OSTimeDly(LONG_TIME);
    }
}

// l-ll
fn task29(_args: *mut c_void) {
    loop {
        // 任务1
        #[cfg(feature = "defmt")]
        info!("---task29 begin---");

        OSTimeDly(LONG_TIME);
    
        #[cfg(feature = "defmt")]
        info!("---task29 end---");
        
        OSTimeDly(LONG_LONG_TIME);
    }
}

// ll-ll
fn task30(_args: *mut c_void) {
    loop {
        // 任务1
        #[cfg(feature = "defmt")]
        info!("---task30 begin---");

        OSTimeDly(LONG_LONG_TIME);
    
        #[cfg(feature = "defmt")]
        info!("---task30 end---");
        
        OSTimeDly(LONG_LONG_TIME);
    }
}