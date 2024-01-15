use std::fs::File;
use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

// Module: filemerger
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
pub struct CRDT { 
    pub sorting: u64,
    pub changes: Vec<String>,
    pub line_range: Range,
}
impl CRDT {
    pub fn compare(&self, other: &CRDT) -> CRDT {
        let clone1: CRDT = self.clone();
        let clone2: CRDT = other.clone();
        let difference: bool = self.sorting > other.sorting;
        let total_difference: i64 = self.sorting as i64 - other.sorting as i64;
        match total_difference {
            0 => {
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
            },
            _ => {
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
}
pub struct CommitMetadata {
    hash: String,
    message: String,
    crdtdata: HashMap<String, CRDT>,
    author: String,
}
impl CommitMetadata {
    pub fn new(hash_as_num: u64, message: String, files: Vec<String>) -> CommitMetadata {
        todo!()
        }
    pub fn compare(&self, other: &CommitMetadata) -> merge_result {
        todo!()
    }
}
pub struct merge_result {
   
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