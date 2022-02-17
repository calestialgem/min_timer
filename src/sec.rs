use std::{
    fmt::Display,
    num::ParseFloatError,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
    str::FromStr,
    time::Duration,
};

/// Time or duration in SI units.
///
/// # Examples
///
/// ## 1 Creation
///
/// Use `new` to create with a `f64`. Or you can convert a `f64`.
/// Consider using `from` when converting a concrete `f64`,
/// and `new` when creating from an temporary `f64` expression.
///
/// ```
/// use min_timer::Sec;
/// const A: f64 = 123.456;
///
/// assert_eq!(Sec::from(A), Sec::new(A));
/// ```
///
/// ## 2 Standard Library Duration
///
/// You can convert to and from [std::time::Duration].
///
/// ```
/// use min_timer::Sec;
/// use std::time::Duration;
/// const A: f64 = 123.456;
///
/// let dur = Duration::from_secs_f64(A);
/// let sec = Sec::from(dur);
///
/// assert_eq!(Sec::from(A), sec);
/// ```
///
/// ## 3 Addition & Subtraction
///
/// You can add or subtract only seconds from each other, not with a scalar `f64`.
///
/// ```
/// use min_timer::Sec;
/// const A: f64 = 123.456;
/// const B: f64 = 666.420;
///
/// let a = Sec::from(A);
/// let b = Sec::from(B);
///
/// assert_eq!(A, a.as_f64());
/// assert_eq!(B, b.as_f64());
/// assert_eq!(Sec::new(A + B), a + b);
/// assert_eq!(Sec::new(A - B), a - b);
/// assert_eq!(Sec::new(B - A), -(a - b));
/// ```
///
/// ## 4 Multiplication & Division
///
/// You can only multiply or divide seconds with a scalar.
/// You cannot multiply two seconds or divide a scalar to a second.
///
/// ```
/// use min_timer::Sec;
/// const A: f64 = 123.456;
/// const B: f64 = 666.420;
/// const C: f64 = 3.33;
///
/// let a = Sec::from(A);
/// let b = Sec::from(B);
///
/// assert_eq!(Sec::new(A * C), a * C);
/// assert_eq!(Sec::new(B / C), b / C);
/// ```
///
/// ## 5 Assignment Operations
///
/// You can mutate in place as well.
///
/// ```
/// use min_timer::Sec;
/// const A: f64 = 123.456;
/// const B: f64 = 666.420;
/// const C: f64 = 3.33;
///
/// let a = Sec::new(A);
/// let b = Sec::from(B);
///
/// let mut c = a;
/// let mut d = a;
/// let mut e = a;
/// let mut f = b;
/// c += b;
/// d -= b;
/// e *= C;
/// f /= C;
///
/// assert_eq!(Sec::new(A + B), c);
/// assert_eq!(Sec::new(A - B), d);
/// assert_eq!(Sec::new(A * C), e);
/// assert_eq!(Sec::new(B / C), f);
/// ```
///
/// ## 6 Parsing & Formatting
///
/// You can parse a string like a `f64`.
/// And you can format to a string with a " s" at the end.
///
/// ```
/// use min_timer::Sec;
/// const A: f64 = 123.456;
///
/// let g = format!("{}", A);
/// let h: Sec = g.parse().unwrap();
///
/// assert_eq!(Sec::from(A), h);
/// assert_eq!(format!("{} s", A), format!("{}", h));
/// ```
///
/// ## 7 Constants
///
/// Multiples and submultiples of seconds from 10^-9 to 10^9.
/// Furthermore; minutes, hours and days also exist.
/// If you find a use case to these please share it with me!
///
/// ```
/// use min_timer::Sec;
///
/// let duration = 50.0 * Sec::MILLI;
/// let same = Sec::new(0.050);
///
/// assert_eq!(duration, same);
/// ```
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, Default)]
pub struct Sec(f64);

impl Sec {
    /// One giga second, Gs.
    pub const GIGA: Sec = Sec::new(1e9);
    /// One day, d.
    pub const DAY: Sec = Sec::new(24.0 * Sec::HOUR.0);
    /// One mega second, Ms.
    pub const MEGA: Sec = Sec::new(1e6);
    /// One hour, h.
    pub const HOUR: Sec = Sec::new(60.0 * Sec::MINUTE.0);
    /// One kilo second, ks.
    pub const KILO: Sec = Sec::new(1e3);
    /// One minute, min.
    pub const MINUTE: Sec = Sec::new(60.0);
    /// One second, s.
    pub const ONE: Sec = Sec::new(1.0);
    /// One milli second, ms.
    pub const MILLI: Sec = Sec::new(1e-3);
    /// One micro second, us.
    pub const MICRO: Sec = Sec::new(1e-6);
    /// One nano second, ns.
    pub const NANO: Sec = Sec::new(1e-9);

    /// Creates from the given amount.
    pub const fn new(amt: f64) -> Self {
        Self(amt)
    }

    /// Returns the amount.
    pub const fn as_f64(&self) -> f64 {
        self.0
    }
}

impl From<f64> for Sec {
    fn from(amt: f64) -> Self {
        Self(amt)
    }
}

impl From<Sec> for f64 {
    fn from(sec: Sec) -> Self {
        sec.0
    }
}

impl From<Duration> for Sec {
    fn from(dur: Duration) -> Self {
        Self(dur.as_secs_f64())
    }
}

impl From<Sec> for Duration {
    fn from(sec: Sec) -> Self {
        Self::from_secs_f64(sec.0)
    }
}

impl Add for Sec {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign for Sec {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0
    }
}

impl Sub for Sec {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl SubAssign for Sec {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0
    }
}

impl Mul<f64> for Sec {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self(self.0 * rhs)
    }
}

impl Mul<Sec> for f64 {
    type Output = Sec;

    fn mul(self, rhs: Sec) -> Self::Output {
        rhs * self
    }
}

impl MulAssign<f64> for Sec {
    fn mul_assign(&mut self, rhs: f64) {
        self.0 *= rhs;
    }
}

impl Div<f64> for Sec {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self(self.0 / rhs)
    }
}

impl DivAssign<f64> for Sec {
    fn div_assign(&mut self, rhs: f64) {
        self.0 /= rhs;
    }
}

impl Neg for Sec {
    type Output = Sec;

    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

impl FromStr for Sec {
    type Err = ParseFloatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse::<f64>()?))
    }
}

impl Display for Sec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} s", self.0)
    }
}
