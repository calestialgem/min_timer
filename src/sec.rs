use std::{
    fmt::Display,
    num::ParseFloatError,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
    str::FromStr,
    time::Duration,
};

/// Time or duration in SI units.
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

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::Sec;

    #[test]
    fn from_double() {
        let double = 123.456;
        let sec = Sec::from(double);
        assert_eq!(double, sec.0);
        let sec: Sec = double.into();
        assert_eq!(double, sec.0);
    }

    #[test]
    fn from_dur() {
        const A: f64 = 1.000_45;
        let dur = Duration::from_secs_f64(A);
        let sec = Sec::from(dur);
        assert_eq!(Sec::from(A), sec);
    }

    #[test]
    fn into_dur() {
        const A: f64 = 1.000_45;
        let dur = Duration::from_secs_f64(A);
        let sec = Sec::from(A);
        assert_eq!(dur, sec.into());
    }

    #[test]
    fn add_and_sub() {
        const A: f64 = 5.0;
        const B: f64 = 0.75;
        let a = Sec::from(A);
        let b = Sec::from(B);
        assert_eq!(Sec::from(A + B), a + b);
        assert_eq!(Sec::from(A - B), a - b);

        let mut c = Sec::from(A);
        let mut d = Sec::from(A);
        c += b;
        d -= b;
        assert_eq!(Sec::from(A + B), c);
        assert_eq!(Sec::from(A - B), d);
    }

    #[test]
    fn mul_div() {
        const A: f64 = 3.33;
        const B: f64 = 11.11;
        let a = Sec::from(A);
        assert_eq!(Sec::from(A * B), a * B);
        assert_eq!(Sec::from(A / B), a / B);

        let mut b = Sec::from(A);
        let mut c = Sec::from(A);
        b *= B;
        c /= B;
        assert_eq!(Sec::from(A * B), b);
        assert_eq!(Sec::from(A / B), c);
    }

    #[test]
    fn neg() {
        const A: f64 = 45.67;
        const B: f64 = 66.66;
        let a = Sec::new(A);
        let b = Sec::new(B);
        assert_eq!(Sec::new(B - A), -(a - b));
    }

    #[test]
    fn from_str_fmt() {
        const A: f64 = 64.45;
        let a = format!("{}", A);
        let b: Sec = a.parse().unwrap();
        assert_eq!(Sec::from(A), b);
        assert_eq!(format!("{} s", A), format!("{}", b));
    }
}
