use crate::port::INT32U;
/// the mod of instant of uC/OS-II kernel
pub mod instant;
/// the mod of duration of uC/OS-II kernel
pub mod duration;
/// the mod of timer of uC/OS-II kernel
pub mod timer;

/// init the Timer as the Systick
pub fn TimerInit() {}

/// an async delay
pub fn OSTimeDly(_ticks: INT32U) {}
