use std::fs::File;
use crate::CommitMetadata;
use std::io::{BufRead, BufReader};

// Module: filemerger
pub fn merge_files(before: &File, after: &File) -> Option<CommitMetadata> {
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
