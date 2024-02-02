use super::commit_abstractions::CommitModification;
use crate::utils::filemanagers::filemanager::read_dir_to_files;
use std::fs::{self, File};
use std::io::BufReader;
use std::io::Read;
use std::path::Path;
pub fn get_head(file: &File) -> Vec<File> {
    let path: &Path = Path::new(".riel/head");
    let head_read: Result<fs::ReadDir, std::io::Error> = fs::read_dir(path);
    read_dir_to_files(head_read).unwrap()
}
pub fn compare_with_head(file: &File, hash: &str, msg: &str, author: &str) -> CommitModification {
    let mut lines_reader: BufReader<&File> = BufReader::new(file);
    let mut lines_str: String = String::new();
    lines_reader.read_to_string(&mut lines_str).unwrap();
    let lines: Vec<&str> = lines_str.lines().collect();
    let file_in_head: Option<File> = get_file_in_head(file);
    if file_in_head.is_none() {
        //TODO: Manage cases where the file is not on the head, maybe all adds
        todo!()
    } else {
        let head_file: File = file_in_head.unwrap();
        let mut head_reader: BufReader<&File> = BufReader::new(&head_file);
        let mut head_str: String = String::new();
        head_reader.read_to_string(&mut head_str).unwrap();
        let head_lines: Vec<&str> = head_str.lines().collect();

        todo!()
    }
}
pub fn get_file_in_head(file: &File) -> Option<File> {
    // should return an instance of the file on the head if it exists
    todo!()
}
