#[allow(dead_code)]
use std::fs::File;
use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use crate::mergers::commitreader::{metadata_to_abstraction, abstraction_to_metadata};
use std::cmp::Ordering;
pub fn testing() {

}
pub fn simple_merge_files(before: &File, after: &File) -> Option<CommitMetadata> {
  let before_lines: Vec<String> = BufReader::new(before)
    .lines()
    .map(|line| line.unwrap())
    .collect();
  let after_lines: Vec<String> = BufReader::new(after)
    .lines()
    .map(|line| line.unwrap())
    .collect();
  let mut diff_lines: Vec<String> = Vec::new();
  for l in after_lines {
    if !(l == "") && !before_lines.contains(&l) {
      diff_lines.push(l);
    }
  }
  println!("{:?}", diff_lines);
  todo!()
}
pub fn generate_commit_metadata(
    hash: &str, 
    msg: String, 
    files: Vec<String>,
    dest: String,
) -> CommitMetadata {
  todo!()
}
#[derive(Clone)]
pub struct CommitModification { 
    pub date: DateTime,
    pub changes: Vec<String>,
}
impl CommitModification {
    pub fn compare(&self, other: &CommitModification) -> CommitModification {
        let clone1: CommitModification = self.clone();
        let clone2: CommitModification = other.clone();
        let difference: bool = self.date > other.date;
        
        if !difference {
            // they are equal
            let hash1: u64 = hash_string(&self.changes.join(""));
            let hash2: u64 = hash_string(&other.changes.join(""));
            fn hash_string(string: &str) -> u64 {
                let mut hasher = DefaultHasher::new();
                string.hash(&mut hasher);
                hasher.finish()
            }
            if hash1 > hash2 {
                return clone1;
            } else {
                return clone2;
            }
        } else {
            // they are not equal
            if difference {
                // self is greater
                return clone1;
            } else {
                // other is greater
                return clone2;
            }
            }
        }
}
pub struct CommitMetadata {
    pub hash: String,
    pub message: String,
    pub author: String,
    pub modifications: Vec<CommitModification>
}
#[derive(Clone)]
pub struct Range {
    pub segments: Vec<(u32, u32)>
}
impl Range {
    pub fn contains(&self, x: u32) -> bool {
        self.segments.iter().any(|&(start, end)| start < x && x < end)
    }
    pub fn add(&mut self, start: u32, end: u32) -> bool {
        if start > end {
            panic!("Range's start was bigger than range's end.");
        }
        if self.segments.iter().any(|&(s, e)| e == start || s == end) {
            panic!("Ranges are not allowed to have the same start or end.");
        }
        if self.contains(start) || self.contains(end) {
            panic!("Tried to add a range with overlapping values. (Fast check)");
        }
        for any in start..end {
            if self.contains(any) {
                panic!("Tried to add a range with overlapping values. (Slow check)");
            }
        }
        self.segments.push((start, end));
        true
    }
    fn get_min(&self) -> u32 {
        let mut min: u32 = 0;
        for &(start, _) in &self.segments {
            if start < min {
                min = start;
            }
        }
        min
    }
    fn get_max(&self) -> u32 {
        let mut max: u32 = 0;
        for &(_, end) in &self.segments {
            if end > max {
                max = end;
            }
        }
        max
    }
}
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
    fn new(year: i32, month: u32, day: u32, hours: u32, minutes: u32, seconds: u32, milliseconds: u32) -> DateTime {
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

    fn compare(&self, other: &DateTime) -> Ordering {
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