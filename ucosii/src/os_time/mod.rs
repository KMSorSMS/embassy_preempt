use crate::port::INT32U;
pub mod instant;
pub mod duration;
pub mod timer;

/// init the Timer as the Systick
pub fn TimerInit() {}

/// an async delay
pub fn OSTimeDly(_ticks: INT32U) {}
