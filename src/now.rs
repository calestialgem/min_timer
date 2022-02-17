use crate::Sec;
use std::time::Instant;

/// Resource that has a time relative to an arbitrary moment.
pub trait Now {
    /// Returns current the time.
    fn now(&self) -> Sec;
}

/// [Now] that uses [standard library](std::time::Instant).
/// Standard library uses [two integers](std::time::Duration), thus you pay for
/// conversion to `f64` all the time.
#[derive(Debug)]
pub struct Std {
    start: Instant,
}

impl Std {
    /// Creates from the current instant.
    pub fn new() -> Self {
        Self {
            start: Instant::now(),
        }
    }
}

impl Now for Std {
    fn now(&self) -> Sec {
        Sec::from(self.start.elapsed())
    }
}
