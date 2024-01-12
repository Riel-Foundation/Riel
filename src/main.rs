#![allow(unused_variables, dead_code, unused_imports, unused_mut, unused_assignments)]
use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::collections::{HashMap, HashSet};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::io;
const RIEL_IGNORE_BUFFER: &[u8] = 
b"# This is a .rielignore file. It is used to ignore files when adding them to the repository.
\n# Folders should be written like this: \n.git\ntest\nignorethisfolder\nnode-modules\ntarget";
const COMMANDS: [&str; 6] = ["help", "mount", "commit", "add", "sudo-destruct", "goto"];
#[derive(Clone)]
struct ParsedArgsObject {
    command: String,
    subcommands: Vec<String>,
    options: Vec<String>,
}
fn main() {
    const RIEL_WORKS: &str = "Riel works! Try help or --help for more information";
    const HELP: &str = "Welcome to Riel!\n Last help message update: 2024-1-11 by Yeray Romero\n Usage: riel ([options]) [command] [arguments/subcommands] \n\nCommands:\nhelp: Shows this message.\nmount: Mounts a Riel repository in the current directory.\ncommit: Commits changes to the repository.\nadd: Adds files to the repository.\nsudo-destruct: For developer purposes, deletes the repository.\ngoto: Goes to a commit, saving local files and not commiting anything yet.\n\nRiel is still in development.\n";
    let args: Vec<String> = env::args().collect();
    if args.contains(&"--help".to_string()) || args.contains(&"help".to_string()) {
        println!("{}", HELP); //TODO: Change this to exec a help command, handling something like riel help add
        return;
    }
    let rielless_args: Vec<String> = args.iter().filter(|x| !x.contains("riel")).map(|x| x.to_string()).collect();
    let executable_args: ParsedArgsObject = parse_args(rielless_args);
    let command: &str = executable_args.command.as_str();
    let subcomands_and_options: ParsedArgsObject = executable_args.clone();
    exec(command, subcomands_and_options)
}
fn parse_args(args: Vec<String>) -> ParsedArgsObject {
    let mut command: String;
    let options: Vec<String> =
    args.iter()
        .filter(|x| x.starts_with("-"))
        .map(|x| x.to_string())
        .collect();
    let possible_commands: Vec<String> =
    args.iter()
        .filter(|x| !x.starts_with("-"))
        .map(|x| x.to_string()).collect();
    let coincidences: i16 = possible_commands.len() as i16;
    match coincidences {
        0 => {
            panic!("No valid command found. Try help or --help for more information.");
        },
        _ => {
            command = possible_commands[0].to_string();
            if !COMMANDS.contains(&command.as_str()) {
                panic!("Commands can only be preceded by options. Try help (this is a command) or --help (this is a ''option'') for more information.");
            }
        }
    }
    let subcommands: Vec<String> = possible_commands.iter().skip(1).map(|x| x.to_string()).collect();
    return ParsedArgsObject {
        command,
        subcommands,
        options,
    };
    
}
fn exec(command: &str, args: ParsedArgsObject) -> () {
    match command {
        "mount" => mount_repo(), // for now, no subcommands,
        "commit" => {
            if commit(args.subcommands) {
                println!("Commited.");
            } else {
                println!("Commit failed.");
            }},
        "add" =>  {
            if add_files(args.subcommands, args.options) {
                println!("Added file(s).");
            } else {
                println!("Could not add all the files.");
            }
        },
        "sudo-destruct" => {
            drop(args.subcommands);
            println!("Riel is still in development. This command could be removed in the future.");
            fs::remove_dir_all(".riel").expect("Failed to remove .riel directory.");
        },
        "goto" => {
            match args.subcommands.len() {
                0 => println!("No commit specified."),
                1 => {
                    prepare_goto(args.subcommands[0].to_string());
                }
                _ => println!("Goto only accepts one argument."),
            }
        },
        _ => println!("Failed to parse command here.")
    }
}
fn mount_repo() -> () {
    if check_repo() {
        println!("Riel repository already exists in this directory.");
    } else {
        create_repo();
        println!("Riel repository created successfully.");
    }
    // TODO: Probably externalize this function
    fn create_repo() -> () {
        const SUBSEQUENT_FAIL_MESSAGE: &str = "Failed to create a directory, but .riel worked. Please check your storage & folder structure.";
        fs::create_dir(".riel").expect("Failed to create .riel directory, please check your permissions.");
        fs::create_dir(".riel/commits").expect(SUBSEQUENT_FAIL_MESSAGE);
        fs::create_dir(".riel/area").expect(SUBSEQUENT_FAIL_MESSAGE);
        fs::create_dir(".riel/commits/local").expect(SUBSEQUENT_FAIL_MESSAGE);
        fs::create_dir(".riel/commits/updated").expect(SUBSEQUENT_FAIL_MESSAGE);
        // create rielignore
        if !fs::metadata(".rielignore").is_ok() {
            let mut ignore_file = fs::File::create("./.rielignore").expect("Failed to create .rielignore file.");
            ignore_file.write_all(RIEL_IGNORE_BUFFER).expect("Failed to write to .rielignore file.");
        }
    }
}
fn file_compress(f: File) -> File {
    // TODO: Compress files to save space AND change this function to another file
    return f;
}
fn file_decompress(fc: File) -> File  {
    return fc;
}
fn commit(commit_args: Vec<String>) -> bool {
    
    // TODO: Ensure CRDT systems works as well as possible and keeping redundancy to a minimum
    let time = std::time::SystemTime::now();
    let time_to_number = time.duration_since(std::time::UNIX_EPOCH).expect("Failed to get time for the hash.");
    let number1 = time_to_number.as_secs();
    let number2 = time_to_number.subsec_nanos() % 23 + 1 / 101;
    let mut hasher = DefaultHasher::new();
    (number1, number2).hash(&mut hasher);
    let hash: u64 = hasher.finish();
    save_commit_locally(hash);
    true
}
fn add_files(subcommands: Vec<String>, options: Vec<String>) -> bool {
    //FIXME: Add a check to see if the files are already in the area
    if !check_repo() {
        println!("No valid Riel repository found. Try init or mount.");
        return false;
    }
    let should_add_all: bool = options.contains(&("-all".to_string())) || options.contains(&("-A".to_string()));
    match subcommands.len() + options.len() {
        0 => println!("No files specified."),
        1.. => {
            if should_add_all {
                let ignored: Ignores = get_ignored();
                if ignored.exists {
                    // should add all files except ignored and .riel
                    let ignore_list: Vec<String> = ignored.files.iter().map(|x| x.to_string()).collect();
                    let fixed_ignore_list: Vec<String> = ignore_list.iter().map(|x| format!("./{}", x)).collect();
                    // TODO: Create a .rielignore parser
                    if copy_to_area(fs::read_dir(".")
                    .expect("Failed to read directory.")
                    .map(|x| x.unwrap().path().display().to_string())
                        .filter(|x| !fixed_ignore_list.contains(x))
                        .collect::<Vec<String>>()) 
                        {
                            // if copied
                            return true;
                        }
                } else {
                    println!("Warning: No .rielignore file found. Adding all files.");
                    // should add all files outside of .riel
                    if copy_to_area(fs::read_dir(".")
                    .expect("Failed to read directory.")
                    .map(|x| x.unwrap().path().display().to_string())
                        .collect::<Vec<String>>())
                        {
                            // if copied
                            return true;
                        }
                }
            }
            else {
                // should add all files specified
                if copy_to_area(subcommands)
                {
                    return true;
                }
            }
        },
        _ => println!("Failed to parse command."),
    }
    false
}
fn check_repo() -> bool {
    fs::metadata(".riel").is_ok() &&
    fs::metadata(".riel/commits").is_ok() &&
    fs::metadata(".riel/area").is_ok()
}
struct Ignores {
    exists: bool,
    files: Vec<String>,
}
fn get_ignored() -> Ignores {
    let mut ignored: Vec<String> = Vec::new();
    let mut exists: bool = false;
    if fs::metadata("./.rielignore").is_ok() {
        exists = true;
        let ignore_file = fs::read_to_string("./.rielignore").expect("Failed to read .rielignore.");
        for line in ignore_file.lines() {
            if line.starts_with("#") {
                continue;
            }
            ignored.push(line.to_string());
        }
    }
    Ignores {
        exists,
        files: ignored,
    }
}
fn copy_to_area(items: Vec<String>) -> bool {
    for item in items {
        if item.starts_with(".riel") || item.starts_with("./.riel") {
            continue;
        }
        if fs::metadata(format!("./{}/", &item)).is_ok() { // directory-folder { // file
            fs::create_dir(format!(".riel/area/{}", &item)).expect("Failed to create directory.");
            println!("Organized {}.", item);
            copy_to_area(fs::read_dir(&item).expect("Failed to read directory.").map(|x| x.unwrap().path().display().to_string()).collect::<Vec<String>>());
        } else if fs::metadata(&item).is_ok() { // file
            fs::copy(&item, format!(".riel/area/{}", &item)).expect("Failed to copy item.");
            println!("Prepared {}.", item);
        }else{
            println!("{} does not exist. Usage is: riel add -A/[file1 file2 file3...]", item);
            return false;
        }
    }
    true
}
fn save_commit_locally(hash: u64) -> bool {
    let hash_str = hash.to_string();
    let hash_reduced = hash_str.chars().take(12).collect::<String>();
    let files_as_strings: Vec<String> = fs::read_dir(".riel/area")
        .expect("Failed to read directory.")
        .map(|x| x.unwrap().path().display().to_string())
        .collect::<Vec<String>>();
    println!("Saving commit locally with hash {}.", hash_reduced);
    area_into_commit(&hash_reduced);
    let data: CommitMetadata = 
    CommitMetadata::new(hash.to_string(),
    "riel does not support messages yet".to_string(), 
    files_as_strings);
    true
}

fn area_into_commit(hash: &str) {
    let src_str: &str = ".riel/area";
    let dest_str = format!(".riel/commits/local/{}", hash);
    fs::create_dir(&dest_str).expect("Failed to create directory.");
    copy_recursive(std::path::Path::new(src_str), std::path::Path::new(&dest_str));
    fs::remove_dir_all(".riel/area").expect("Failed to remove area.");
    fs::create_dir(".riel/area").expect("Failed to create area.");
}
fn prepare_goto(hash: String) -> bool {
    let hash_reduced = hash;
    let dest_str = format!(".riel/commits/local/{}", hash_reduced);
    if !fs::metadata(&dest_str).is_ok() {
        println!("Commit {} does not exist, check your hash or use riel load (Not developed yet)", hash_reduced); //FIXME
        return false;
    }
    copy_recursive(std::path::Path::new(&dest_str), std::path::Path::new("."));
    true
}
fn copy_recursive(src: &std::path::Path, dest: &std::path::Path) -> bool {
    if let Err(e) = copy_directory(src, dest) {
        eprintln!("Failed to copy directory: {}", e);
        return false;
    }

    true
}
fn copy_directory(src: &std::path::Path, dest: &std::path::Path) -> io::Result<()> {
    if src.is_dir() {
        fs::create_dir_all(dest)?;

        for entry in fs::read_dir(src)? {
            let entry = entry?;
            let entry_type = entry.file_type()?;

            let new_dest = dest.join(entry.file_name());

            if entry_type.is_dir() {
                copy_recursive(&entry.path(), &new_dest);
            } else {
                fs::copy(&entry.path(), &new_dest)?;
            }
        }
    }

    Ok(())
}
#[derive(Clone)]
struct Range {
    segments: Vec<(u32, u32)>
}
impl Range {
    fn contains(&self, value: u32) -> bool {
        self.segments.iter().any(|&(start, end)| start <= value && value <= end)
    }
    fn add(&mut self, start: u32, end: u32) -> bool {
        if start > end {
            panic!("Range's start was bigger than range's end.");
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
#[derive(Clone)] // Would be nice to implement Copy
/**
 * Conflict-free replicated data type: https://en.wikipedia.org/wiki/Conflict-free_replicated_data_type
 */
struct CRDT { 
    sorting: u64,
    changes: Vec<String>,
    line_range: Range,
}
impl CRDT {
    fn compare(&self, other: &CRDT) -> CRDT {
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
struct CommitMetadata {
    hash: String,
    message: String,
    crdtdata: HashMap<String, CRDT>,
}
impl CommitMetadata {
    fn new(hash: String, message: String, files: Vec<String>) -> CommitMetadata {
        let mut crdtdata: HashMap<String, CRDT> = HashMap::new();
        for file in files {
            let changes = crdt_get_changes(&file);
            if changes.file_found && changes.file_changed {
                if changes.data.is_some() {
                    crdtdata.insert(file, changes.data.unwrap());
                }else {
                    panic!("Failed to get changes for file {}.", file);
                }
            }else {
                if !changes.file_found {
                    crdtdata.insert(file, CRDT {
                        sorting: 0,
                        changes: Vec::new(),
                        line_range: Range {
                            segments: vec![(0, 1)]
                        }
                    });
                }
            }
        }
        CommitMetadata {
            hash,
            message,
            crdtdata,
        }
    }
}
struct CrdtFileStateObject {
    file_found: bool,
    file_changed: bool,
    data: Option<CRDT>,
}
fn crdt_get_changes(file: &str) -> CrdtFileStateObject {
    todo!()
}
// Tests for Range & CRDT 
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_range_contains() {
    let r: Range = Range {
        segments: vec![(0, 10), (20, 30), (40, 50)]
    };
    assert_eq!(r.contains(0), true);
    assert_eq!(r.contains(10), true);
    assert_eq!(r.contains(20), true);
    assert_eq!(r.contains(30), true);
    assert_eq!(r.contains(40), true);
    assert_eq!(r.contains(50), true);
    assert_eq!(r.contains(11), false);
    assert_eq!(r.contains(19), false);
    assert_eq!(r.contains(31), false);
    assert_eq!(r.contains(39), false);
    assert_eq!(r.contains(51), false);
    assert_eq!(r.contains(100), false);
    assert_eq!(r.contains(8), true);
}
#[test]
#[should_panic(expected = "Tried to add a range with overlapping values. (Fast check)")]
fn test_range_fail() {
    let mut r: Range = Range {
        segments: vec![(0, 10), (20, 30), (40, 50)]
    };
    r.add(8, 12);
}
#[test]
#[should_panic(expected = "Tried to add a range with overlapping values. (Fast check)")]
fn test_range_fail2() {
    let mut r: Range = Range {
        segments: vec![(0, 10), (20, 30), (40, 50)]
    };
    r.add(10, 21);
}
#[test]
#[should_panic(expected = "Tried to add a range with overlapping values. (Slow check)")]
fn test_range_fail3() {
    let mut r: Range = Range {
        segments: vec![(0, 10), (20, 30), (40, 50)]
    };
    r.add(11, 55);
}
#[test]
#[should_panic(expected = "Range's start was bigger than range's end.")]
fn test_range_all() {
    let mut r: Range = Range {
        segments: vec![(0, 10), (20, 30), (40, 50)]
    };
    assert_eq!(r.add(11, 19), true);
    r.add(60, 75);
    assert_eq!(r.get_min(), 0);
    assert_eq!(r.get_max(), 75);
    assert_eq!(r.contains(65), true);
    assert_eq!(r.add(51, 59), true);
    r.add(100, 80);
}
#[test]
#[should_panic(expected = "Tried to add a range with overlapping values. (Fast check)")]
fn test_range_same_values() {
    let mut r: Range = Range {
        segments: vec![(0, 10), (20, 30), (40, 50)]
    };
    r.add(10, 20);
}
// Testing CRDT
#[test]
fn test_crdt1() {
    let crdt1: CRDT = CRDT {
        sorting: 77,
        changes: vec!["Hello".to_string(), "World".to_string()],
        line_range: Range {
            segments: vec![(0, 1), (2, 3)]
        }
    };
    let crdt2: CRDT = CRDT {
        sorting: 88,
        changes: vec!["Hello".to_string(), "World".to_string()],
        line_range: Range {
            segments: vec![(0, 1), (2, 3)]
        }
    };
    assert_eq!(crdt1.compare(&crdt2).sorting, 88);
}
#[test]
fn test_crdt_reliability() {
    let crdt1: CRDT = CRDT {
        sorting: 99,
        changes: vec!["Hello".to_string()],
        line_range: Range {
            segments: vec![(0, 1), (2, 3)]
        }
    };
    let crdt2: CRDT = CRDT {
        sorting: 99,
        changes: vec!["One".to_string()],
        line_range: Range {
            segments: vec![(0, 1), (2, 3)]
        }
    };
    assert_eq!(crdt1.compare(&crdt2).changes[0], crdt2.compare(&crdt1).changes[0]);
}
#[test]
fn test_crdt_reliability2() {
    let crdt1: CRDT = CRDT {
        sorting: 199,
        changes: vec!["h".to_string()],
        line_range: Range {
            segments: vec![(0, 1), (2, 3)]
        }
    };
    let crdt2: CRDT = CRDT {
        sorting: 199,
        changes: vec!["1".to_string()],
        line_range: Range {
            segments: vec![(0, 1), (2, 3)]
        }
    };
    assert_eq!(crdt1.compare(&crdt2).changes[0], crdt2.compare(&crdt1).changes[0]);
} 
}

// If last two test work, it's a matter of fact that if all commits have enough metadata,
  // Conflicts shouldn't be too hard to avoid
  // That does not mean code will not get broken, but it's easier for a developer
  // to focus on fix broken code that to fix a broken Version Control System feature.
  // Of course, it will be always better to not have two people working on the same file
  // Just in case I will make a local copy of the file the user has local changes on
  // TODO: This is the EOF but I don't like having comments in code, this should be removed.