use core::{future::Future, ops::Add, pin::Pin, task::{Context, Poll}};

use crate::port::INT32U;
#[allow(unused)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
/// An Instant in time, based on the MCU's clock ticks since startup.
pub struct Instant {
    ticks: u64,
}
impl Instant {
    /// The smallest (earliest) value that can be represented by the `Instant` type.
    pub const MIN: Instant = Instant { ticks: u64::MIN };
    /// The largest (latest) value that can be represented by the `Instant` type.
    pub const MAX: Instant = Instant { ticks: u64::MAX };

    /// Returns an Instant representing the current time.
    pub fn now() -> Instant {
        Instant {
            // by noah: in stage one, we set the tick as zero
            ticks: 0,
            // ticks: embassy_time_driver::now(),
        }
    }
    /// Adds one Duration to self, returning a new `Instant` or None in the event of an overflow.
    pub fn checked_add(&self, duration: Duration) -> Option<Instant> {
        self.ticks.checked_add(duration.ticks).map(|ticks| Instant { ticks })
    }
}

impl Add<Duration> for Instant {
    type Output = Instant;

    fn add(self, other: Duration) -> Instant {
        self.checked_add(other)
            .expect("overflow when adding duration to instant")
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
/// Represents the difference between two [Instant](struct.Instant.html)s
pub struct Duration {
    pub(crate) ticks: u64,
}

impl Duration {
    // /// The smallest value that can be represented by the `Duration` type.
    // pub const MIN: Duration = Duration { ticks: u64::MIN };
    // /// The largest value that can be represented by the `Duration` type.
    // pub const MAX: Duration = Duration { ticks: u64::MAX };

    // /// Tick count of the `Duration`.
    // pub const fn as_ticks(&self) -> u64 {
    //     self.ticks
    // }

    // /// Convert the `Duration` to seconds, rounding down.
    // pub const fn as_secs(&self) -> u64 {
    //     self.ticks / TICK_HZ
    // }

    // /// Convert the `Duration` to milliseconds, rounding down.
    // pub const fn as_millis(&self) -> u64 {
    //     self.ticks * (1000 / GCD_1K) / (TICK_HZ / GCD_1K)
    // }

    // /// Convert the `Duration` to microseconds, rounding down.
    // pub const fn as_micros(&self) -> u64 {
    //     self.ticks * (1_000_000 / GCD_1M) / (TICK_HZ / GCD_1M)
    // }

    /// Creates a duration from the specified number of clock ticks
    pub const fn from_ticks(ticks: u64) -> Duration {
        Duration { ticks }
    }

    /// Adds one Duration to another, returning a new Duration or None in the event of an overflow.
    pub fn checked_add(self, rhs: Duration) -> Option<Duration> {
        self.ticks.checked_add(rhs.ticks).map(|ticks| Duration { ticks })
    }
}

impl Add for Duration {
    type Output = Duration;

    fn add(self, rhs: Duration) -> Duration {
        self.checked_add(rhs).expect("overflow when adding durations")
    }
}

#[allow(unused)]
/// A future that completes at a specified [Instant](struct.Instant.html).
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct Timer {
    expires_at: Instant,
    yielded_once: bool,
}

impl Future for Timer {
    type Output = ();
    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        // if self.yielded_once && self.expires_at <= Instant::now() {
        //     Poll::Ready(())
        // } else {
        //     embassy_time_queue_driver::schedule_wake(self.expires_at.as_ticks(), cx.waker());
        //     self.yielded_once = true;
        //     Poll::Pending
        // }
        self.yielded_once = true;
        Poll::Pending
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

    // /// Expire after the specified number of microseconds.
    // ///
    // /// This method is a convenience wrapper for calling `Timer::after(Duration::from_micros())`.
    // /// For more details, refer to [`Timer::after()`] and [`Duration::from_micros()`].
    // #[inline]
    // pub fn after_micros(micros: u64) -> Self {
    //     Self::after(Duration::from_micros(micros))
    // }

    // /// Expire after the specified number of milliseconds.
    // ///
    // /// This method is a convenience wrapper for calling `Timer::after(Duration::from_millis())`.
    // /// For more details, refer to [`Timer::after`] and [`Duration::from_millis()`].
    // #[inline]
    // pub fn after_millis(millis: u64) -> Self {
    //     Self::after(Duration::from_millis(millis))
    // }

    // /// Expire after the specified number of seconds.
    // ///
    // /// This method is a convenience wrapper for calling `Timer::after(Duration::from_secs())`.
    // /// For more details, refer to [`Timer::after`] and [`Duration::from_secs()`].
    // #[inline]
    // pub fn after_secs(secs: u64) -> Self {
    //     Self::after(Duration::from_secs(secs))
    // }
}

/// an async delay
pub async fn OSTimeDly(_ticks:INT32U){
     
}