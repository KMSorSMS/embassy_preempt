use core::{future::Future, pin::Pin, task::{Context, Poll}};

use crate::executor::waker;

use super::BOT_DRIVER;

/// a bottom
pub struct bottom {
    // the bottom can just await once
    yielded_once: bool,
}

impl Future for bottom{
    type Output = ();
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.yielded_once {
            Poll::Ready(())
        } else {
            // add the bottom(task) to the bottom driver
            BOT_DRIVER.set_task(waker::task_from_waker(cx.waker()));
            self.yielded_once = true;
            Poll::Pending
        }
    }
}

impl bottom {
    /// return a bottom which is waiting for the rising edge(only support this now)
    /// in the future, we can put the interrupt code here to support different bottom(rising edge, falling edge, etc.)
    pub fn wait_for_rising_edge() -> Self {
        Self {
            yielded_once: false,
        }
    }
}