use core::ops::Add;
use crate::cfg::TICK_HZ;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
/// Represents the difference between two [Instant](struct.Instant.html)s
pub struct Duration {
    pub(crate) ticks: usize,
}
const fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
pub(crate) const GCD_1K: usize = gcd(TICK_HZ, 1_000);

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
    pub const fn from_ticks(ticks: usize) -> Duration {
        Duration { ticks }
    }

    /// Creates a duration from the specified number of milliseconds, rounding up.
    pub const fn from_millis(millis: usize) -> Duration {
        Duration {
            ticks: div_ceil(millis * (TICK_HZ / GCD_1K), 1000 / GCD_1K),
        }
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

#[inline]
const fn div_ceil(num: usize, den: usize) -> usize {
    (num + den - 1) / den
}
