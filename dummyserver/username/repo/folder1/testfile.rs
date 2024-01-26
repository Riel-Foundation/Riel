use std::{cmp::Ordering, time::SystemTime};

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
    pub fn now() -> DateTime {
        let base = std::time::SystemTime::now();
        DateTime::from(base)
    }
    fn from(time: SystemTime) -> DateTime {
       if let Ok(duration) = time.duration_since(SystemTime::UNIX_EPOCH) {
            let seconds: u64 = duration.as_secs();
            let milliseconds: u32 = duration.subsec_millis();

            let (years, remaining_seconds) = (seconds / 31536000, seconds % 31536000);
            let (months, remaining_seconds) = (remaining_seconds / 2592000, remaining_seconds % 2592000);
            let (days, remaining_seconds) = (remaining_seconds / 86400, remaining_seconds % 86400);
            let (hours, remaining_seconds) = (remaining_seconds / 3600, remaining_seconds % 3600);
            let (minutes, remaining_seconds) = (remaining_seconds / 60, remaining_seconds % 60);

            DateTime::new(
                1970 + years as i32,
                months as u32 + 1,
                days as u32 + 1,
                hours as u32,
                minutes as u32,
                remaining_seconds as u32,
                milliseconds,
            )
        } else {
            panic!("SystemTime is earlier than UNIX epoch!");
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