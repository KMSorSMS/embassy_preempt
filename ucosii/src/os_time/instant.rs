use core::ops::Add;
use super::duration::Duration;
#[allow(unused)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
/// An Instant in time, based on the MCU's clock ticks since startup.
pub struct Instant {
    ticks: usize,
}
impl Instant {
    /// The smallest (earliest) value that can be represented by the `Instant` type.
    pub const MIN: Instant = Instant { ticks: usize::MIN };
    /// The largest (latest) value that can be represented by the `Instant` type.
    pub const MAX: Instant = Instant { ticks: usize::MAX };

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

