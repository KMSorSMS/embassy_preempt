use core::marker::PhantomData;
use super::raw::OS_TCB_REF;
pub struct SpawnToken<S> {
    raw_task: Option<OS_TCB_REF>,
    phantom: PhantomData<*mut S>,
}
impl<S> SpawnToken<S> {
    pub(crate) unsafe fn new(raw_task: OS_TCB_REF) -> Self {
        Self {
            raw_task: Some(raw_task),
            phantom: PhantomData,
        }
    }

    /// Return a SpawnToken that represents a failed spawn.
    pub fn new_failed() -> Self {
        Self {
            raw_task: None,
            phantom: PhantomData,
        }
    }
}