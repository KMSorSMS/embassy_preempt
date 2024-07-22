//! Raw task storage and pool.

use super::OS_TCB_REF;
#[cfg_attr(feature = "cortex_m", path = "state_atomics_arm.rs")]
pub mod state;
pub mod waker;

/// Wake a task by `TaskRef`.
///
/// You can obtain a `TaskRef` from a `Waker` using [`task_from_waker`].
pub fn wake_task(task: OS_TCB_REF) {
    let header = task.header();
    if header.OSTCBStat.run_enqueue() {
        // We have just marked the task as scheduled, so enqueue it.
        unsafe {
            let executor = header.executor.get().unwrap_unchecked();
            executor.enqueue(task);
        }
    }
}
