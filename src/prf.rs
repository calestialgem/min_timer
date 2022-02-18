use crate::{now::Now, Sec, Timer};
use std::ops::AddAssign;

/// Smart pointer, which automaticly accumulates the time it takes to drop.
/// It can be used to profile whole scopes just by creating.
///
/// # Example
///
/// ```
/// use min_timer::{Std, Prf, Stat};
///
/// fn subroutine() {}
///
/// let mut stat = Stat::new();
/// let now = Std::new();
///
/// { let _ = Prf::new(&now, &mut stat); subroutine(); }
/// { let _ = Prf::new(&now, &mut stat); subroutine(); }
///
/// stat.refresh();
///
/// { let _ = Prf::new(&now, &mut stat); subroutine(); }
/// { let _ = Prf::new(&now, &mut stat); subroutine(); }
///
/// assert_eq!(4, stat.count());
/// assert_eq!(2, stat.rate());
/// ```
pub struct Prf<'a, T: Now, U: AddAssign<Sec>> {
    timer: Timer<'a, T>,
    acc: &'a mut U,
}

impl<'a, T: Now, U: AddAssign<Sec>> Prf<'a, T, U> {
    /// Creates with a new timer.
    pub fn new(now: &'a T, acc: &'a mut U) -> Self {
        Self {
            timer: Timer::new(now),
            acc,
        }
    }
}

impl<'a, T: Now, U: AddAssign<Sec>> Drop for Prf<'a, T, U> {
    fn drop(&mut self) {
        *self.acc += self.timer.elapsed();
    }
}
