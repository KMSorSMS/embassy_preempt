//! Waker implementation for uC/OS-II.
use core::mem;
use core::task::{RawWaker, RawWakerVTable, Waker};

use super::{wake_task, OS_TCB, OS_TCB_REF};

static VTABLE: RawWakerVTable = RawWakerVTable::new(clone, wake, wake, drop);

unsafe fn clone(p: *const ()) -> RawWaker {
    RawWaker::new(p, &VTABLE)
}

unsafe fn wake(p: *const ()) {
    wake_task(OS_TCB_REF::from_ptr(p as *const OS_TCB))
}

unsafe fn drop(_: *const ()) {
    // nop
}

pub(crate) unsafe fn from_task(p: OS_TCB_REF) -> Waker {
    Waker::from_raw(RawWaker::new(p.as_ptr() as _, &VTABLE))
}

/// Get a task pointer from a waker.
///
/// This can be used as an optimization in wait queues to store task pointers
/// (1 word) instead of full Wakers (2 words). This saves a bit of RAM and helps
/// avoid dynamic dispatch.
///
/// You can use the returned task pointer to wake the task with [`wake_task`](super::wake_task).
///
/// # Panics
///
/// Panics if the waker is not created by the Embassy executor.
pub fn task_from_waker(waker: &Waker) -> OS_TCB_REF {
    // safety: OK because WakerHack has the same layout as Waker.
    // This is not really guaranteed because the structs are `repr(Rust)`, it is
    // indeed the case in the current implementation.
    // TODO use waker_getters when stable. https://github.com/rust-lang/rust/issues/96992
    let hack: &WakerHack = unsafe { mem::transmute(waker) };
    if hack.vtable != &VTABLE {
        panic!("Found waker not created by the Embassy executor. `embassy_time::Timer` only works with the Embassy executor.")
    }

    // safety: our wakers are always created with `OS_TCB_REF::as_ptr`
    unsafe { OS_TCB_REF::from_ptr(hack.data as *const OS_TCB) }
}

struct WakerHack {
    data: *const (),
    vtable: &'static RawWakerVTable,
}
