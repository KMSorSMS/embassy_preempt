/// Alarm handle, assigned by the driver.
#[derive(Clone, Copy)]
pub struct AlarmHandle {
    id: u8,
}
/// Time driver
pub trait Driver: Send + Sync + 'static {
    /// Return the current timestamp in ticks.
    ///
    /// Implementations MUST ensure that:
    /// - This is guaranteed to be monotonic, i.e. a call to now() will always return
    ///   a greater or equal value than earler calls. Time can't "roll backwards".
    /// - It "never" overflows. It must not overflow in a sufficiently long time frame, say
    ///   in 10_000 years (Human civilization is likely to already have self-destructed
    ///   10_000 years from now.). This means if your hardware only has 16bit/32bit timers
    ///   you MUST extend them to 64-bit, for example by counting overflows in software,
    ///   or chaining multiple timers together.
    fn now(&self) -> u64;

    /// Try allocating an alarm handle. Returns None if no alarms left.
    /// Initially the alarm has no callback set, and a null `ctx` pointer.
    ///
    /// # Safety
    /// It is UB to make the alarm fire before setting a callback.
    unsafe fn allocate_alarm(&self) -> Option<AlarmHandle>;

    /// Sets the callback function to be called when the alarm triggers.
    /// The callback may be called from any context (interrupt or thread mode).
    fn set_alarm_callback(&self, alarm: AlarmHandle, callback: fn(*mut ()), ctx: *mut ());

    /// Sets an alarm at the given timestamp. When the current timestamp reaches the alarm
    /// timestamp, the provided callback function will be called.
    ///
    /// The `Driver` implementation should guarantee that the alarm callback is never called synchronously from `set_alarm`.
    /// Rather - if `timestamp` is already in the past - `false` should be returned and alarm should not be set,
    /// or alternatively, the driver should return `true` and arrange to call the alarm callback as soon as possible, but not synchronously.
    /// There is a rare third possibility that the alarm was barely in the future, and by the time it was enabled, it had slipped into the
    /// past.  This is can be detected by double-checking that the alarm is still in the future after enabling it; if it isn't, `false`
    /// should also be returned to indicate that the callback may have been called already by the alarm, but it is not guaranteed, so the
    /// caller should also call the callback, just like in the more common `false` case. (Note: This requires idempotency of the callback.)
    ///
    /// When callback is called, it is guaranteed that now() will return a value greater or equal than timestamp.
    ///
    /// Only one alarm can be active at a time for each AlarmHandle. This overwrites any previously-set alarm if any.
    fn set_alarm(&self, alarm: AlarmHandle, timestamp: u64) -> bool;
}