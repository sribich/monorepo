use std::{
    fmt::{Debug, Display, Formatter},
    ops::{Add, AddAssign, Neg, Sub, SubAssign},
};

/// Represents a single point in time, used for subtitle timing ranges.
#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Time(i64);

pub type TimePoint = Time;
pub type TimeDelta = Time;

impl Time {
    pub fn from_components(hours: i64, minutes: i64, seconds: i64, milliseconds: i64) -> Time {
        let minutes_combined = minutes + hours * 60;
        let seconds_combined = seconds + minutes_combined * 60;
        let milliseconds_combined = milliseconds + seconds_combined * 1000;

        Time(milliseconds_combined)
    }

    fn hours(&self) -> i64 {
        self.0 / (60 * 60 * 1000)
    }

    fn mins(&self) -> i64 {
        self.0 / (60 * 1000)
    }

    fn secs(&self) -> i64 {
        self.0 / 1000
    }

    fn msecs(&self) -> i64 {
        self.0
    }

    fn mins_comp(&self) -> i64 {
        self.mins() % 60
    }

    fn secs_comp(&self) -> i64 {
        self.secs() % 60
    }

    fn msecs_comp(&self) -> i64 {
        self.msecs() % 1000
    }
}

impl Debug for Time {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Timing({})", self.to_string())
    }
}

impl Display for Time {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let time = if self.0 < 0 { -*self } else { *self };

        write!(
            f,
            "{}{}:{:02}:{:02}.{:03}",
            if self.0 < 0 { "-" } else { "" },
            time.hours(),
            time.mins_comp(),
            time.secs_comp(),
            time.msecs_comp()
        )
    }
}

impl Add for Time {
    type Output = Time;

    fn add(self, rhs: Time) -> Time {
        Time(self.0 + rhs.0)
    }
}

impl Sub for Time {
    type Output = Time;

    fn sub(self, rhs: Time) -> Time {
        Time(self.0 - rhs.0)
    }
}

impl AddAssign for Time {
    fn add_assign(&mut self, r: Time) {
        self.0 += r.0;
    }
}

impl SubAssign for Time {
    fn sub_assign(&mut self, r: Time) {
        self.0 -= r.0;
    }
}

impl Neg for Time {
    type Output = Time;

    fn neg(self) -> Time {
        Time(-self.0)
    }
}

#[derive(Clone, Debug)]
pub struct TimeSpan {
    pub start: TimePoint,
    pub end: TimePoint,
}

impl TimeSpan {
    pub fn new(start: TimePoint, end: TimePoint) -> TimeSpan {
        TimeSpan { start, end }
    }

    pub fn len(&self) -> TimeDelta {
        self.end - self.start
    }
}

impl Add<TimeDelta> for TimeSpan {
    type Output = TimeSpan;
    fn add(self, rhs: TimeDelta) -> TimeSpan {
        TimeSpan::new(self.start + rhs, self.end + rhs)
    }
}

impl Sub<TimeDelta> for TimeSpan {
    type Output = TimeSpan;
    fn sub(self, rhs: TimeDelta) -> TimeSpan {
        TimeSpan::new(self.start - rhs, self.end - rhs)
    }
}

impl AddAssign<TimeDelta> for TimeSpan {
    fn add_assign(&mut self, r: TimeDelta) {
        self.start += r;
        self.end += r;
    }
}

impl SubAssign<TimeDelta> for TimeSpan {
    fn sub_assign(&mut self, r: TimeDelta) {
        self.start -= r;
        self.end -= r;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_timing_display() {
        let t = -super::Time::from_components(12, 59, 29, 450);
        assert_eq!(t.to_string(), "-12:59:29.450".to_string());

        // let t = super::Time::from_msecs(0);
        // assert_eq!(t.to_string(), "0:00:00.000".to_string());
    }
}
