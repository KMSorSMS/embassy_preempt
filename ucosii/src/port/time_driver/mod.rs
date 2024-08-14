use crate::executor::waker;

/// Schedule the given waker to be woken at `at`.
pub fn _embassy_time_schedule_wake(at: u64, waker: &core::task::Waker){
    let task = waker::task_from_waker(waker);
    let task = task.header();
    unsafe {
        let expires_at = task.expires_at.get();
        task.expires_at.set(expires_at.min(at));
    }
}