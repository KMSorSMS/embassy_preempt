use crate::port::INT32U;
#[allow(unused)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
/// An Instant in time, based on the MCU's clock ticks since startup.
pub struct Instant {
    ticks: u64,
}

#[allow(unused)]
/// A future that completes at a specified [Instant](struct.Instant.html).
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct Timer {
    expires_at: Instant,
    yielded_once: bool,
}

// impl Timer {
//     /// Expire at specified [Instant](struct.Instant.html)
//     pub fn at(expires_at: Instant) -> Self {
//         Self {
//             expires_at,
//             yielded_once: false,
//         }
//     }

//     /// Expire after specified [Duration](struct.Duration.html).
//     /// This can be used as a `sleep` abstraction.
//     ///
//     /// Example:
//     /// ``` no_run
//     /// use embassy_time::{Duration, Timer};
//     ///
//     /// #[embassy_executor::task]
//     /// async fn demo_sleep_seconds() {
//     ///     // suspend this task for one second.
//     ///     Timer::after(Duration::from_secs(1)).await;
//     /// }
//     /// ```
//     pub fn after(duration: Duration) -> Self {
//         Self {
//             expires_at: Instant::now() + duration,
//             yielded_once: false,
//         }
//     }

//     /// Expire after the specified number of ticks.
//     ///
//     /// This method is a convenience wrapper for calling `Timer::after(Duration::from_ticks())`.
//     /// For more details, refer to [`Timer::after()`] and [`Duration::from_ticks()`].
//     #[inline]
//     pub fn after_ticks(ticks: u64) -> Self {
//         Self::after(Duration::from_ticks(ticks))
//     }

//     /// Expire after the specified number of nanoseconds.
//     ///
//     /// This method is a convenience wrapper for calling `Timer::after(Duration::from_nanos())`.
//     /// For more details, refer to [`Timer::after()`] and [`Duration::from_nanos()`].
//     #[inline]
//     pub fn after_nanos(nanos: u64) -> Self {
//         Self::after(Duration::from_nanos(nanos))
//     }

//     /// Expire after the specified number of microseconds.
//     ///
//     /// This method is a convenience wrapper for calling `Timer::after(Duration::from_micros())`.
//     /// For more details, refer to [`Timer::after()`] and [`Duration::from_micros()`].
//     #[inline]
//     pub fn after_micros(micros: u64) -> Self {
//         Self::after(Duration::from_micros(micros))
//     }

//     /// Expire after the specified number of milliseconds.
//     ///
//     /// This method is a convenience wrapper for calling `Timer::after(Duration::from_millis())`.
//     /// For more details, refer to [`Timer::after`] and [`Duration::from_millis()`].
//     #[inline]
//     pub fn after_millis(millis: u64) -> Self {
//         Self::after(Duration::from_millis(millis))
//     }

//     /// Expire after the specified number of seconds.
//     ///
//     /// This method is a convenience wrapper for calling `Timer::after(Duration::from_secs())`.
//     /// For more details, refer to [`Timer::after`] and [`Duration::from_secs()`].
//     #[inline]
//     pub fn after_secs(secs: u64) -> Self {
//         Self::after(Duration::from_secs(secs))
//     }
// }

/// an async delay
pub async fn OSTimeDly(_ticks:INT32U){
     
}