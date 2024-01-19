use std::fs::{self, File};
use std::io::Read;
use std::path::Path;
use std::io::BufReader;
use crate::read_dir_to_files;
use crate::mergers::commit_abstractions::CommitMetadata;

pub fn get_head(file: &File) -> Vec<File> {
    let path: &Path = Path::new(".riel/head");
    let head_read: Result<fs::ReadDir, std::io::Error> = fs::read_dir(path);
    read_dir_to_files(head_read).unwrap()
}
pub fn compare_with_head(file: &File) -> CommitMetadata {
    let mut lines_reader: BufReader<&File> = BufReader::new(file);
    let mut lines_str: String = String::new();
    lines_reader.read_to_string(&mut lines_str).unwrap();
    let lines: Vec<&str> = lines_str.lines().collect();
    todo!()
}