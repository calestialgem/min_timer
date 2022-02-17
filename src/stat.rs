use crate::Sec;
use std::ops::AddAssign;

/// Time statistics of a subroutine.
///
/// # Example
///
/// ```
/// use min_timer::{Sec, Stat};
///
/// let mut s = Stat::new();
///
/// s += Sec::new(3.0); // subroutine took 3 s
///
/// assert_eq!(1, s.get_rate());
///
/// s.refresh(); // a new cycle starts
///
/// s += Sec::new(5.0); // subroutine took 5 s
///
/// assert_eq!(Sec::new(4.0), s.find_average());
/// assert_eq!(2, s.get_count());
/// assert_eq!(1, s.get_rate());
/// ```
#[derive(Debug, Clone, Copy, Default)]
pub struct Stat {
    total: Sec,
    count: u64,
    rate: u64,
}

impl Stat {
    /// Creates clean.
    pub fn new() -> Self {
        Self::default()
    }

    /// Means the end of a cycle.
    /// Rate is calculated based on this.
    ///
    /// For example the render statistics in a game engine can be refreshed every second.
    /// This way the `rate` will be the FPS counter.
    pub fn refresh(&mut self) {
        self.rate = 0;
    }

    /// Returns the average duration of the subroutine.
    pub fn find_average(&self) -> Sec {
        self.total / self.count as f64
    }

    /// Returns the total amount of times the subroutine was called.
    pub fn get_count(&self) -> u64 {
        self.count
    }

    /// Returns the amount of times the subroutine was called in the previous cycle.
    /// The cycles end with a refresh call.
    pub fn get_rate(&self) -> u64 {
        self.rate
    }
}

impl AddAssign<Sec> for Stat {
    fn add_assign(&mut self, rhs: Sec) {
        self.total += rhs;
        self.count += 1;
        self.rate += 1;
    }
}
