use crate::{now::Now, Sec};
use std::ops::{Add, AddAssign, Sub};

/// Finds time relative to the moment it is created.
///
/// # Example
///
/// ```
/// use std::time::{Duration, Instant};
/// use min_timer::{now::Std, Sec, Timer};
///
/// let now = Std::new();
/// let timer = Timer::new(&now);
/// let duration = 5.0 * Sec::MILLI;
///
/// // For stopping an infinite loop if the test fails.
/// let start = Instant::now();
/// let cap = Duration::from(duration + Sec::MILLI);
///
/// while timer < duration {
///     if start.elapsed() > cap {
///         unreachable!("Passed {}!", duration);
///     }
/// }
///
/// let tolerance = 100.0 * Sec::MICRO;
/// assert!(timer - duration < tolerance);
/// ```
#[derive(Debug)]
pub struct Timer<'a, T: Now> {
    start: Sec,
    now: &'a T,
}

impl<'a, T: Now> Clone for Timer<'a, T> {
    fn clone(&self) -> Self {
        Self {
            start: self.start,
            now: self.now,
        }
    }
}

impl<'a, T: Now> Copy for Timer<'a, T> {}

impl<'a, T: Now> Timer<'a, T> {
    /// Creates starting from this moment.
    pub fn new(now: &'a T) -> Self {
        Self {
            start: now.now(),
            now,
        }
    }

    /// Returns the elapsed time.
    pub fn elapsed(&self) -> Sec {
        self.now.now() - self.start
    }
}

impl<'a, T: Now> Add<Sec> for Timer<'a, T> {
    type Output = Self;

    fn add(self, rhs: Sec) -> Self::Output {
        Self {
            start: self.start + rhs,
            now: self.now,
        }
    }
}

impl<'a, T: Now> AddAssign<Sec> for Timer<'a, T> {
    fn add_assign(&mut self, rhs: Sec) {
        self.start += rhs;
    }
}

impl<'a, T: Now> Sub<Sec> for Timer<'a, T> {
    type Output = Sec;

    fn sub(self, rhs: Sec) -> Self::Output {
        self.elapsed() - rhs
    }
}

impl<'a, T: Now> Sub<Timer<'a, T>> for Sec {
    type Output = Sec;

    fn sub(self, rhs: Timer<'a, T>) -> Self::Output {
        self - rhs.elapsed()
    }
}

impl<'a, T: Now> PartialEq<Sec> for Timer<'a, T> {
    fn eq(&self, other: &Sec) -> bool {
        self.elapsed() == *other
    }
}

impl<'a, T: Now> PartialOrd<Sec> for Timer<'a, T> {
    fn partial_cmp(&self, other: &Sec) -> Option<std::cmp::Ordering> {
        self.elapsed().partial_cmp(other)
    }
}
