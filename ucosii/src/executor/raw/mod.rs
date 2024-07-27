//! Raw task storage and pool.

use run_queue_atomics::RunQueue;

use super::OS_TCB_REF;
#[cfg_attr(feature = "cortex_m", path = "state_atomics_arm.rs")]
pub mod state;
pub mod waker;
mod run_queue_atomics;


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


#[derive(Clone, Copy)]
pub(crate) struct Pender(*mut ());

unsafe impl Send for Pender {}
unsafe impl Sync for Pender {}

impl Pender {
    pub(crate) fn pend(self) {
        extern "Rust" {
            fn __pender(context: *mut ());
        }
        unsafe { __pender(self.0) };
    }
}


pub(crate) struct SyncExecutor {
    run_queue: RunQueue,
    pender: Pender,
}

impl SyncExecutor {
    pub(crate) fn new(pender: Pender) -> Self {
        #[cfg(feature = "integrated-timers")]
        let alarm = unsafe { unwrap!(embassy_time_driver::allocate_alarm()) };

        Self {
            run_queue: RunQueue::new(),
            pender,
        }
    }

    /// Enqueue a task in the task queue
    ///
    /// # Safety
    /// - `task` must be a valid pointer to a spawned task.
    /// - `task` must be set up to run in this executor.
    /// - `task` must NOT be already enqueued (in this executor or another one).
    #[inline(always)]
    unsafe fn enqueue(&self, task: OS_TCB_REF) {
        if self.run_queue.enqueue(task) {
            self.pender.pend();
        }
    }

    pub(super) unsafe fn spawn(&'static self, task: OS_TCB_REF) {
        task.header().executor.set(Some(self));

        self.enqueue(task);
    }

    /// # Safety
    ///
    /// Same as [`Executor::poll`], plus you must only call this on the thread this executor was created.
    pub(crate) unsafe fn poll(&'static self) {
        #[allow(clippy::never_loop)]
        loop {
            self.run_queue.dequeue_all(|p| {
                let task = p.header();


                if !task.state.run_dequeue() {
                    // If task is not running, ignore it. This can happen in the following scenario:
                    //   - Task gets dequeued, poll starts
                    //   - While task is being polled, it gets woken. It gets placed in the queue.
                    //   - Task poll finishes, returning done=true
                    //   - RUNNING bit is cleared, but the task is already in the queue.
                    return;
                }


                // Run the task
                task.poll_fn.get().unwrap_unchecked()(p);

            });

            {
                break;
            }
        }
    }
}

