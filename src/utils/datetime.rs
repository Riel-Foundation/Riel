use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq, PartialOrd, Clone)]
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
}

impl Ord for DateTime {
    fn cmp(&self, other: &DateTime) -> Ordering {
        self.year.cmp(&other.year)
            .then_with(|| self.month.cmp(&other.month))
            .then_with(|| self.day.cmp(&other.day))
            .then_with(|| self.hours.cmp(&other.hours))
            .then_with(|| self.minutes.cmp(&other.minutes))
            .then_with(|| self.seconds.cmp(&other.seconds))
            .then_with(|| self.milliseconds.cmp(&other.milliseconds))
    }
}