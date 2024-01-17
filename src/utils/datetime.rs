use std::cmp::Ordering;
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct DateTime {
    year: i32,
    month: u32,
    day: u32,
    hours: u32,
    minutes: u32,
    seconds: u32,
    milliseconds: u32,
}

impl DateTime {
    pub fn new(year: i32, month: u32, day: u32, hours: u32, minutes: u32, seconds: u32, milliseconds: u32) -> DateTime {
        DateTime {
            year,
            month,
            day,
            hours,
            minutes,
            seconds,
            milliseconds,
        }
    }

    pub fn compare(&self, other: &DateTime) -> Ordering {
        if self.year != other.year {
            self.year.cmp(&other.year)
        } else if self.month != other.month {
            self.month.cmp(&other.month)
        } else if self.day != other.day {
            self.day.cmp(&other.day)
        } else if self.hours != other.hours {
            self.hours.cmp(&other.hours)
        } else if self.minutes != other.minutes {
            self.minutes.cmp(&other.minutes)
        } else if self.seconds != other.seconds {
            self.seconds.cmp(&other.seconds)
        } else {
            self.milliseconds.cmp(&other.milliseconds)
        }
    }
    fn clone(&self) -> DateTime {
        DateTime {
            year: self.year,
            month: self.month,
            day: self.day,
            hours: self.hours,
            minutes: self.minutes,
            seconds: self.seconds,
            milliseconds: self.milliseconds,
        }
    }
}