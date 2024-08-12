use core::{future::Future, pin::Pin, task::{Context, Poll, Waker}};

use super::{duration::Duration, instant::Instant};

#[allow(unused)]
/// A future that completes at a specified [Instant](struct.Instant.html).
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct Timer {
    expires_at: Instant,
    yielded_once: bool,
}

extern "Rust" {
    fn _embassy_time_schedule_wake(at: usize, waker: &Waker);
}

/// Schedule the given waker to be woken at `at`.
pub fn schedule_wake(at: usize, waker: &Waker) {
    unsafe { _embassy_time_schedule_wake(at, waker) }
}

impl Future for Timer {
    type Output = ();
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.yielded_once && self.expires_at <= Instant::now() {
            Poll::Ready(())
        } else {
            schedule_wake(self.expires_at.as_ticks(), cx.waker());           
            self.yielded_once = true;
            Poll::Pending
        }
        // self.yielded_once = true;
        // Poll::Pending
    }
}

impl Timer {
    /// Expire at specified [Instant](struct.Instant.html)
    pub fn at(expires_at: Instant) -> Self {
        Self {
            expires_at,
            yielded_once: false,
        }
    }

    /// Expire after specified [Duration](struct.Duration.html).
    /// This can be used as a `sleep` abstraction.
    ///
    /// Example:
    /// ``` no_run
    /// use embassy_time::{Duration, Timer};
    ///
    /// #[embassy_executor::task]
    /// async fn demo_sleep_seconds() {
    ///     // suspend this task for one second.
    ///     Timer::after(Duration::from_secs(1)).await;
    /// }
    /// ```
    pub fn after(duration: Duration) -> Self {
        Self {
            expires_at: Instant::now() + duration,
            yielded_once: false,
        }
    }

    /// Expire after the specified number of ticks.
    ///
    /// This method is a convenience wrapper for calling `Timer::after(Duration::from_ticks())`.
    /// For more details, refer to [`Timer::after()`] and [`Duration::from_ticks()`].
    #[inline]
    pub fn after_ticks(ticks: usize) -> Self {
        Self::after(Duration::from_ticks(ticks))
    }

    // /// Expire after the specified number of nanoseconds.
    // ///
    // /// This method is a convenience wrapper for calling `Timer::after(Duration::from_nanos())`.
    // /// For more details, refer to [`Timer::after()`] and [`Duration::from_nanos()`].
    // #[inline]
    // pub fn after_nanos(nanos: u64) -> Self {
    //     Self::after(Duration::from_nanos(nanos))
    // }

    /// Expire after the specified number of microseconds.
    ///
    /// This method is a convenience wrapper for calling `Timer::after(Duration::from_micros())`.
    /// For more details, refer to [`Timer::after()`] and [`Duration::from_micros()`].
    #[inline]
    pub fn after_micros(micros: usize) -> Self {
        Self::after(Duration::from_micros(micros))
    }

    /// Expire after the specified number of milliseconds.
    ///
    /// This method is a convenience wrapper for calling `Timer::after(Duration::from_millis())`.
    /// For more details, refer to [`Timer::after`] and [`Duration::from_millis()`].
    #[inline]
    pub fn after_millis(millis: usize) -> Self {
        Self::after(Duration::from_millis(millis))
    }

    /// Expire after the specified number of seconds.
    ///
    /// This method is a convenience wrapper for calling `Timer::after(Duration::from_secs())`.
    /// For more details, refer to [`Timer::after`] and [`Duration::from_secs()`].
    #[inline]
    pub fn after_secs(secs: usize) -> Self {
        Self::after(Duration::from_secs(secs))
    }
}
