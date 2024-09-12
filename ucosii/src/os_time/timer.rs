use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll, Waker};
#[cfg(feature = "alarm_test")]
use defmt::trace;

use super::duration::Duration;
use super::instant::Instant;

#[allow(unused)]
/// A future that completes at a specified [Instant](struct.Instant.html).
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct Timer {
    expires_at: Instant,
    yielded_once: bool,
}

extern "Rust" {
    fn _embassy_time_schedule_wake(at: u64, waker: &Waker);
}

/// Schedule the given waker to be woken at `at`.
pub fn schedule_wake(at: u64, waker: &Waker) {
    unsafe { _embassy_time_schedule_wake(at, waker) }
}

impl Unpin for Timer {}

impl Future for Timer {
    type Output = ();
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.yielded_once && self.expires_at <= Instant::now() {
            self.yielded_once = false;
            #[cfg(feature = "alarm_test")]
            trace!("Timer expired");
            Poll::Ready(())
        } else {
            // by noah:this func set the expire time of the task
            schedule_wake(self.expires_at.as_ticks(), cx.waker());
            self.yielded_once = true;
            #[cfg(feature = "alarm_test")]
            trace!("Set wake at {}", self.expires_at.as_ticks());
            Poll::Pending
        }
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
    pub fn after_ticks(ticks: u64) -> Self {
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
    pub fn after_micros(micros: u64) -> Self {
        Self::after(Duration::from_micros(micros))
    }

    /// Expire after the specified number of milliseconds.
    ///
    /// This method is a convenience wrapper for calling `Timer::after(Duration::from_millis())`.
    /// For more details, refer to [`Timer::after`] and [`Duration::from_millis()`].
    #[inline]
    pub fn after_millis(millis: u64) -> Self {
        let tmp = Self::after(Duration::from_millis(millis));
        #[cfg(feature = "alarm_test")]
        // 打印timer信息
        trace!("the timer{:?}'s yield_once is {:?} , yield_once address is {}, timer_address is {}", millis, tmp.yielded_once, &tmp.yielded_once as *const _, &tmp as *const _);
        tmp
    }

    /// Expire after the specified number of seconds.
    ///
    /// This method is a convenience wrapper for calling `Timer::after(Duration::from_secs())`.
    /// For more details, refer to [`Timer::after`] and [`Duration::from_secs()`].
    #[inline]
    pub fn after_secs(secs: u64) -> Self {
        Self::after(Duration::from_secs(secs))
    }
}
