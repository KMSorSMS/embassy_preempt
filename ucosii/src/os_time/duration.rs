use core::ops::Add;

use crate::cfg::TICK_HZ;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
/// Represents the difference between two [Instant](struct.Instant.html)s
pub struct Duration {
    pub(crate) ticks: u64,
}
const fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
pub(crate) const GCD_1K: u64 = gcd(TICK_HZ, 1_000);
pub(crate) const GCD_1M: u64 = gcd(TICK_HZ, 1_000_000);

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

    /// Creates a duration from the specified number of seconds, rounding up.
    pub const fn from_secs(secs: u64) -> Duration {
        Duration { ticks: secs * TICK_HZ }
    }

    /// Creates a duration from the specified number of milliseconds, rounding up.
    pub const fn from_millis(millis: u64) -> Duration {
        Duration {
            ticks: div_ceil(millis * (TICK_HZ / GCD_1K), 1000 / GCD_1K),
        }
    }

    /// Creates a duration from the specified number of microseconds, rounding up.
    /// NOTE: Delays this small may be inaccurate.
    pub const fn from_micros(micros: u64) -> Duration {
        Duration {
            ticks: div_ceil(micros * (TICK_HZ / GCD_1M), 1_000_000 / GCD_1M),
        }
    }

    /// Creates a duration from the specified number of seconds, rounding down.
    pub const fn from_secs_floor(secs: u64) -> Duration {
        Duration { ticks: secs * TICK_HZ }
    }

    /// Creates a duration from the specified number of milliseconds, rounding down.
    pub const fn from_millis_floor(millis: u64) -> Duration {
        Duration {
            ticks: millis * (TICK_HZ / GCD_1K) / (1000 / GCD_1K),
        }
    }

    /// Creates a duration from the specified number of microseconds, rounding down.
    /// NOTE: Delays this small may be inaccurate.
    pub const fn from_micros_floor(micros: u64) -> Duration {
        Duration {
            ticks: micros * (TICK_HZ / GCD_1M) / (1_000_000 / GCD_1M),
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
const fn div_ceil(num: u64, den: u64) -> u64 {
    (num + den - 1) / den
}
