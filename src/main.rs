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
\n# Folders should be written like this: \n.git\ntest\nignorethisfolder\nnode-modules";
const COMMANDS: [&str; 4] = ["mount", "commit", "add", "sudo-destruct"];
#[derive(Clone)]
struct ParsedArgsObject {
    command: String,
    subcommands: Vec<String>,
    options: Vec<String>,
}
fn main() {
    const RIEL_WORKS: &str = "Riel works! Try help or --help for more information";
    const HELP: &str = "Welcome to Riel! Try help or --help for more information, or init / create to start a repository.";
    let args: Vec<String> = env::args().collect();
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
        1.. => {
            command = possible_commands[0].to_string();
            if !COMMANDS.contains(&command.as_str()) {
                panic!("Commands can only be preceded by options. Try help or --help for more information.");
            }
        },
        _ => {
            panic!("Too many commands found. Try help or --help for more information.");
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
            if add_files(args.subcommands) {
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
        let mut ignore_file = fs::File::create("./.rielignore").expect("Failed to create .rielignore file.");
        ignore_file.write_all(RIEL_IGNORE_BUFFER).expect("Failed to write to .rielignore file.");
    }
}
fn file_compress(f: File) -> File {
    // TODO: Compress files to save space AND change this function to another file
    return f;
}
fn commit(commit_args: Vec<String>) -> bool {
    
    // TODO: Ensure CRDT systems works as well as possible and keeping redundancy to a minimum
    let time = std::time::SystemTime::now();
    let time_to_number = time.duration_since(std::time::UNIX_EPOCH).expect("Failed to get time for the hash.");
    let number1 = time_to_number.as_secs();
    let number2 = time_to_number.subsec_nanos() % 23 + 1 / 101;
    let mut hasher = DefaultHasher::new();
    (number1, number2).hash(&mut hasher);
    let hash = hasher.finish();
    save_commit_locally(hash);
    let files_saved: Vec<String> = fs::read_dir(".riel/commits/local").expect("Failed to read directory.").map(|x| x.unwrap().path().display().to_string()).collect::<Vec<String>>();


    true
}
fn add_files(subcommands: Vec<String>) -> bool {
    //FIXME: Add a check to see if the files are already in the area
    if !check_repo() {
        println!("No valid Riel repository found. Try init or mount.");
        return false;
    }
    let checking_vec = subcommands.iter().map(|x| x.as_str()).collect::<Vec<&str>>();
    let should_add_all: bool = checking_vec.contains(&"-all") || checking_vec.contains(&"-A");
    match subcommands.len() {
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
    println!("Saving commit locally with hash {}.", hash_reduced);
    let src_str: &str = ".riel/area";
    let dest_str = format!(".riel/commits/local/{}", hash_reduced);
    fs::create_dir(&dest_str).expect("Failed to create directory.");
    copy_recursive(std::path::Path::new(src_str), std::path::Path::new(&dest_str));
    fs::remove_dir_all(".riel/area").expect("Failed to remove area.");
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
            return false;
        }
        if self.contains(start) || self.contains(end) {
            eprintln!("Tried to add a range with overlapping values.");
            return false;
        }
        self.segments.push((start, end));
        true
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
        let they_are_equal_saver: u64 = self.sorting % 2;
        let difference: bool = self.sorting > other.sorting;
        let total_difference: u64 = self.sorting - other.sorting;
        match total_difference {
            0 => {
                // they are equal
                if they_are_equal_saver == 0 {
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
    files: Vec<String>,
    message: String,
    crdtdata: CRDT,
}