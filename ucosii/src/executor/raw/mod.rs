//! Raw task storage and pool.
use core::future::Future;
use crate::executor::spawner::SpawnToken;

use super::{AvailableTask, OS_TASK_STORAGE};

/// Raw storage that can hold up to N tasks of the same type.
///
/// This is essentially a `[OS_TASK_STORAGE<F>; N]`.
pub struct TaskPool<F: Future + 'static, const N: usize> {
    pool: [OS_TASK_STORAGE<F>; N],
}

impl<F: Future + 'static, const N: usize> TaskPool<F, N> {
    /// Create a new TaskPool, with all tasks in non-spawned state.
    pub const fn new() -> Self {
        Self {
            pool: [OS_TASK_STORAGE::NEW; N],
        }
    }

    fn spawn_impl<T>(&'static self, future: impl FnOnce() -> F) -> SpawnToken<T> {
        match self.pool.iter().find_map(AvailableTask::claim) {
            Some(task) => task.initialize_impl::<T>(future),
            None => SpawnToken::new_failed(),
        }
    }

    /// Try to spawn a task in the pool.
    ///
    /// See [`OS_TASK_STORAGE::spawn()`] for details.
    ///
    /// This will loop over the pool and spawn the task in the first storage that
    /// is currently free. If none is free, a "poisoned" SpawnToken is returned,
    /// which will cause [`Spawner::spawn()`](super::Spawner::spawn) to return the error.
    pub fn spawn(&'static self, future: impl FnOnce() -> F) -> SpawnToken<impl Sized> {
        self.spawn_impl::<F>(future)
    }

    /// Like spawn(), but allows the task to be send-spawned if the args are Send even if
    /// the future is !Send.
    ///
    /// Not covered by semver guarantees. DO NOT call this directly. Intended to be used
    /// by the Embassy macros ONLY.
    ///
    /// SAFETY: `future` must be a closure of the form `move || my_async_fn(args)`, where `my_async_fn`
    /// is an `async fn`, NOT a hand-written `Future`.
    #[doc(hidden)]
    pub unsafe fn _spawn_async_fn<FutFn>(&'static self, future: FutFn) -> SpawnToken<impl Sized>
    where
        FutFn: FnOnce() -> F,
    {
        // See the comment in AvailableTask::__initialize_async_fn for explanation.
        self.spawn_impl::<FutFn>(future)
    }
}
