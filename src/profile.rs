use crate::{now::Now, Sec, Timer};
use std::ops::AddAssign;

/// Smart pointer, which automaticly accumulates the time it takes to drop.
/// It can be used to profile whole scopes just by creating.
///
/// # Example
///
/// ```
/// use min_timer::{now::Std, Profile, Stat};
///
/// fn subroutine() {}
///
/// let mut stat = Stat::new();
/// let now = Std::new();
///
/// { let _ = Profile::new(&now, &mut stat); subroutine(); }
/// { let _ = Profile::new(&now, &mut stat); subroutine(); }
///
/// stat.refresh();
///
/// { let _ = Profile::new(&now, &mut stat); subroutine(); }
/// { let _ = Profile::new(&now, &mut stat); subroutine(); }
///
/// assert_eq!(4, stat.get_count());
/// assert_eq!(2, stat.get_rate());
/// ```
pub struct Profile<'a, T: Now, U: AddAssign<Sec>> {
    timer: Timer<'a, T>,
    acc: &'a mut U,
}

impl<'a, T: Now, U: AddAssign<Sec>> Profile<'a, T, U> {
    /// Creates with a new timer.
    pub fn new(now: &'a T, acc: &'a mut U) -> Self {
        Self {
            timer: Timer::new(now),
            acc,
        }
    }
}

impl<'a, T: Now, U: AddAssign<Sec>> Drop for Profile<'a, T, U> {
    fn drop(&mut self) {
        *self.acc += self.timer.elapsed();
    }
}
