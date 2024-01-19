#![allow(unused_variables, dead_code, unused_imports, unused_mut, unused_assignments)]
use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::io;
mod mergers;
mod utils;
mod args_parser;
use args_parser::{parse_args, ParsedArgsObject};
const RIEL_IGNORE_BUFFER: &[u8] = 
b"# This is a .rielignore file. It is used to ignore files when adding them to the repository.
\n# Folders should be written like this: \n.git\ntest\nignorethisfolder\nnode-modules\ntarget";
const COMMANDS: [&str; 8] = //TODO: Could this be a HashSet?
["help", "mount", "add",
"commit", "sudo-destruct", "goto", 
"version", "mergetest"];
const RIEL_WORKS: &str = "Riel works! Try help or --help for more information";
const VERSION: &str = "0.0.35";
const HELP: &str = "Welcome to Riel!\n Last help message update: 2024-1-11 by Yeray Romero\n Usage: riel ([options]) [command] [arguments/subcommands] \n\nCommands:\nhelp: Shows this message.\nmount: Mounts a Riel repository in the current directory.\ncommit: Commits changes to the repository.\nadd: Adds files to the repository.\nsudo-destruct: For developer purposes, deletes the repository.\ngoto: Goes to a commit, saving local files and not commiting anything yet.\n\nRiel is still in development.\n";

fn main() {
    // filemerger::testing();
    
    let args: Vec<String> = env::args().collect();
    if args.contains(&"--help".to_string()) || args.contains(&"help".to_string()) {
        println!("{}", HELP); //TODO: Change this to exec a help command, handling something like riel help add
        return;
    }
    let rielless_args: Vec<String> = args.iter().filter(|x| !x.contains("riel")).map(|x| x.to_string()).collect();
    let executable_args: Option<ParsedArgsObject> = parse_args(rielless_args);
    if executable_args.is_none() {
        return;
    }else {
        let executable_args: ParsedArgsObject = executable_args.unwrap();
        let command: String = executable_args.command();
        let subcomands_and_options: ParsedArgsObject = executable_args.clone();
        /*println!("Command: {}", command);
        println!("Subcommands: {:?}", subcomands_and_options.subcommands());
        println!("Options: {:?}", subcomands_and_options.options());*/
        exec(&command, subcomands_and_options)
    }
}

fn exec(command: &str, args: ParsedArgsObject) -> () {
    match command {
        "mount" => mount_repo(), // for now, no subcommands,
        "commit" => {
            if commit(args.options(), args.subcommands()) {
                println!("Commited.");
            } else {
                println!("Commit failed.");
            }},
        "add" =>  {
            if add_files(args.subcommands(), args.options()) {
                println!("Added file(s).");
            } else {
                println!("Could not add all the files.");
            }
        },
        "sudo-destruct" => {
            drop(args.subcommands());
            println!("Riel is still in development. This command could be removed in the future.");
            fs::remove_dir_all(".riel").expect("Failed to remove .riel directory.");
        },
        "goto" => {
            match args.subcommands().len() {
                0 => println!("No commit specified."),
                1 => {
                    prepare_goto(args.subcommands()[0].to_string());
                }
                _ => println!("Goto only accepts one argument."),
            }
        },
        "version" => println!("Riel v{}.", VERSION),
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
        fs::create_dir(".riel/head").expect(SUBSEQUENT_FAIL_MESSAGE);
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
    }
    false
}
fn check_repo() -> bool {
    fs::metadata(".riel").is_ok() &&
    fs::metadata(".riel/commits").is_ok() &&
    fs::metadata(".riel/area").is_ok()
}
pub struct Ignores {
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
fn commit(options: Vec<String>, commit_args: Vec<String>) -> bool { 
    let message_option: String = "-m".to_string();
    let messaged: bool = options.contains(&message_option);
    let msg: String = if messaged {
    let index: usize = options.iter().position(|x| x == &message_option).unwrap();
    commit_args[index].to_string()
    } else {
    "No message provided.".to_string()
    };

    
    // TODO: Ensure CRDT systems works as well as possible and keeping redundancy to a minimum
    let time = std::time::SystemTime::now();
    let time_to_number = time.duration_since(std::time::UNIX_EPOCH).expect("Failed to get time for the hash.");
    let number1 = time_to_number.as_secs();
    let number2 = time_to_number.subsec_nanos() % 101 + 1 / 23;
    let mut hasher = DefaultHasher::new();
    (number1, number2).hash(&mut hasher);
    let hash: u64 = hasher.finish();
    save_commit_locally(hash, &msg)
}
fn save_commit_locally(hash: u64, msg: &str) -> bool {
    let hash_str = hash.to_string();
    let hash_reduced = hash_str.chars().take(12).collect::<String>();
    println!("Saving commit locally with hash {}.", hash_reduced);
    let msg = msg.to_string();
    let src_str: &str = ".riel/area";
    let dest_str = format!(".riel/commits/local/{}", hash);
    let files: Vec<String> = fs::read_dir(".riel/area")
        .expect("Failed to read directory.")
        .map(|x| x.unwrap().path().display().to_string())
        .collect::<Vec<String>>();
    fs::create_dir(&dest_str).expect("Failed to create directory.");
    copy_recursive(std::path::Path::new(src_str), std::path::Path::new(&dest_str));
    fs::remove_dir_all(".riel/area").expect("Failed to remove area.");
    fs::create_dir(".riel/area").expect("Failed to create area.");
    //Check if head is empty
    let is_first_commit: bool = check_head();
    if is_first_commit {
        // copy also to head
        copy_recursive(std::path::Path::new(&dest_str), std::path::Path::new(".riel/head"));
    }else {
        // else, we have to go through crdt and merge
        let head_dir = fs::read_dir(".riel/head");
        let head_files = read_dir_to_files(head_dir).expect("Failed to read directory.");
        //TODO
    }
    true
}
fn check_head() -> bool {
    let head_dir = fs::read_dir(".riel/head").expect("Failed to read directory.");
    let head_files: Vec<fs::DirEntry> = head_dir
        .map(|x| x.unwrap())
        .collect::<Vec<fs::DirEntry>>();
    head_files.len() == 0

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
fn read_dir_to_files(dir_result: Result<fs::ReadDir, std::io::Error>) -> Result<Vec<File>, String> {
    let dir_entries = dir_result.map_err(|e| format!("Error reading directory: {}", e))?;

    let files: Vec<File> = dir_entries
        .filter_map(|entry| {
            if let Ok(entry) = entry {
                if entry.file_type().map_or(false, |ft| ft.is_file()) {
                    if let Ok(file) = File::open(entry.path()) {
                        return Some(file);
                    }
                }
            }
            None
        })
        .collect();

    Ok(files)
}