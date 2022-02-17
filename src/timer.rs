use crate::{now::Now, Sec};
use std::ops::{Add, AddAssign, Sub};

/// Finds time relative to the moment it is created.
#[derive(Debug)]
pub struct Timer<'a, T>
where
    T: Now,
{
    start: Sec,
    now: &'a T,
}

impl<'a, T> Timer<'a, T>
where
    T: Now,
{
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

impl<'a, T> Add<Sec> for Timer<'a, T>
where
    T: Now,
{
    type Output = Self;

    fn add(self, rhs: Sec) -> Self::Output {
        Self {
            start: self.start + rhs,
            now: self.now,
        }
    }
}

impl<'a, T> AddAssign<Sec> for Timer<'a, T>
where
    T: Now,
{
    fn add_assign(&mut self, rhs: Sec) {
        self.start += rhs;
    }
}

impl<'a, T> Sub<Sec> for Timer<'a, T>
where
    T: Now,
{
    type Output = Sec;

    fn sub(self, rhs: Sec) -> Self::Output {
        self.elapsed() - rhs
    }
}

impl<'a, T> Sub<Timer<'a, T>> for Sec
where
    T: Now,
{
    type Output = Sec;

    fn sub(self, rhs: Timer<'a, T>) -> Self::Output {
        self - rhs.elapsed()
    }
}

impl<'a, T> PartialEq<Sec> for Timer<'a, T>
where
    T: Now,
{
    fn eq(&self, other: &Sec) -> bool {
        self.elapsed() == *other
    }
}

impl<'a, T> PartialOrd<Sec> for Timer<'a, T>
where
    T: Now,
{
    fn partial_cmp(&self, other: &Sec) -> Option<std::cmp::Ordering> {
        self.elapsed().partial_cmp(other)
    }
}

#[cfg(test)]
mod tests {
    use std::time::{Duration, Instant};

    use crate::{now::Std, Sec, Timer};

    #[test]
    fn waiting_loop() {
        let clk = Std::new();
        let timer = Timer::new(&clk);
        let duration = 5.0 * Sec::MILLI;
        let start = Instant::now(); // For limiting the waiting.
        let cap = Duration::from(duration + Sec::MILLI);
        while timer < duration {
            if start.elapsed() > cap {
                unreachable!("Passed {}!", duration);
            }
        }
        assert!(timer - duration < Sec::MICRO);
    }
}
