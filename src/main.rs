#![allow(
    unused_variables,
    unused_imports,
    dead_code,
    unused_mut,
    unused_assignments
)]
//std
use std::collections::hash_map::DefaultHasher;
use std::env;
use std::fs;
use std::fs::create_dir;
use std::fs::remove_dir_all;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io;
use std::io::Read;
use std::io::Write;
use std::net::TcpStream;
use std::path::Path;
//mods
mod adding;
mod args_parser;
mod mergers;
mod remotes;
mod utils;
//internal
use adding::add::add_files;
use args_parser::{parse_args, ParsedArgsObject};
use remotes::tcp_web::web_get_with_url;
use utils::filemanagers::filemanager::copy_recursive;

use crate::utils::filemanagers::filemanager::read_dir_to_files;
// consts
const RIEL_IGNORE_BUFFER: &[u8] =
    b"# This is a .rielignore file. It is used to ignore files when adding them to the repository.
\n# Folders should be written like this: \n.git\ntest\nignorethisfolder\nnode-modules\ntarget";
const COMMANDS: [&str; 8] = //TODO: Could this be a HashSet?
    [
        "help",
        "mount",
        "add",
        "commit",
        "sudo-destruct",
        "goto",
        "version",
        "clone",
    ];
const RIEL_WORKS: &str = "Riel works! Try help or --help for more information";
const VERSION: &str = "0.1.0";
const HELP: &str = "Welcome to Riel!
Last help message update: 2024-1-28 by Yeray Romero
Usage: riel ([options]) [command] [arguments/subcommands]\n
Commands:
help: Shows this message.
mount: Mounts a Riel repository in the current directory.
commit: Commits changes to the repository.
add: Adds files to the repository.
clone: Clones a repository from a given URL.
goto: Goes to a commit, saving local files and not commiting anything yet.\n
sudo-destruct: For developer purposes, deletes the repository.\n
Remember Riel is still in development.";

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.contains(&"--help".to_string()) || args.contains(&"help".to_string()) {
        println!("{}", HELP); //TODO: Change this to exec a help command, handling something like riel help add
        return;
    }
    let rielless_args: Vec<String> = args
        .iter()
        .filter(|x| !x.contains("riel"))
        .map(|x| x.to_string())
        .collect();
    let executable_args: Option<ParsedArgsObject> = parse_args(rielless_args);
    if executable_args.is_none() {
        return;
    } else {
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
    // always-working commands: help, version, clone
    if command == "help" {
        println!("{}", HELP);
        return;
    }
    if command == "clone" {
        clone(args.subcommands(), args.options());
        return;
    }
    if command == "version" {
        println!("Riel v{}.", VERSION);
        return;
    }
    match check_repo() {
        true => {
            if command == "mount" {
                println!("Riel repository already exists in this directory.");
                return;
            }
            // clone IS allowed because it will create a new repo inside the current one
            // TODO: Subrepo support
        }
        false => {
            match command {
                "mount" => mount_repo(), // for now, no subcommands,
                _ => {
                    println!("Riel repository does not exist in this directory.");
                    return;
                }
            }
        }
    }
    match command {
        "commit" => {
            if commit(args.options(), args.subcommands()) {
                println!("Commited.");
            } else {
                println!("Commit failed.");
            }
        }
        "add" => {
            if add_files(args.subcommands(), args.options()) {
                println!("Added file(s).");
            } else {
                println!("Could not add all the files.");
            }
        }
        "sudo-destruct" => {
            drop(args.subcommands());
            println!("Riel is still in development. This command could be removed in the future.");
            fs::remove_dir_all(".riel").expect("Failed to remove .riel directory.");
        }
        "goto" => match args.subcommands().len() {
            0 => println!("No commit specified."),
            1 => {
                prepare_goto(args.subcommands()[0].to_string());
            }
            _ => println!("Goto only accepts one argument."),
        },
        _ => println!("Failed to parse command here."),
    }
}
fn clone(subcommands: Vec<String>, options: Vec<String>) -> () {
    if options.len() > 0 {
        println!("Clone does not accept options.");
        return;
    }
    println!("Argument passed: {:?}", subcommands);
    if subcommands.len() < 1 || subcommands.len() > 1 {
        println!("Clone only accepts exactly one argument: the URL.");
        return;
    }
    let stream_option: Option<TcpStream> = web_get_with_url(format!("{}", subcommands[0]).as_str());
    if let Some(mut stream) = stream_option {
        if create_clone_files(&mut stream) {
            println!("Clone successful.");
        } else {
            println!("Clone failed.");
        }
    } else {
        println!("Something went wrong while cloning. Please check your URL.");
    }
}
fn create_clone_files(clone_response: &mut TcpStream) -> bool {
    remotes::tcp_web::receive_directory_structure(clone_response, ".")
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
        fs::create_dir(".riel")
            .expect("Failed to create .riel directory, please check your permissions.");
        fs::create_dir(".riel/commits").expect(SUBSEQUENT_FAIL_MESSAGE);
        fs::create_dir(".riel/area").expect(SUBSEQUENT_FAIL_MESSAGE);
        fs::create_dir(".riel/commits/local").expect(SUBSEQUENT_FAIL_MESSAGE);
        fs::create_dir(".riel/commits/updated").expect(SUBSEQUENT_FAIL_MESSAGE);
        fs::create_dir(".riel/head").expect(SUBSEQUENT_FAIL_MESSAGE);
        // create rielignore
        if !fs::metadata(".rielignore").is_ok() {
            let mut ignore_file =
                fs::File::create("./.rielignore").expect("Failed to create .rielignore file.");
            ignore_file
                .write_all(RIEL_IGNORE_BUFFER)
                .expect("Failed to write to .rielignore file.");
        }
    }
}
fn file_compress(f: File) -> File {
    // TODO: Compress files to save space AND change this function to another file
    return f;
}
fn file_decompress(fc: File) -> File {
    return fc;
}
fn check_repo() -> bool {
    fs::metadata(".riel").is_ok()
        && fs::metadata(".riel/commits").is_ok()
        && fs::metadata(".riel/area").is_ok()
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
    let time_to_number = time
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Failed to get time for the hash.");
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
    create_dir(&dest_str).expect("Failed to create directory.");
    copy_recursive(
        std::path::Path::new(src_str),
        std::path::Path::new(&dest_str),
    );
    remove_dir_all(".riel/area").expect("Failed to remove area.");
    create_dir(".riel/area").expect("Failed to create area.");
    //Check if head is empty
    let is_first_commit: bool = check_head();
    if is_first_commit {
        // copy also to head
        copy_recursive(
            std::path::Path::new(&dest_str),
            std::path::Path::new(".riel/head"),
        );
    } else {
        // else, we have to go through crdt and merge
        let head_dir = fs::read_dir(".riel/head");
        let head_files = read_dir_to_files(head_dir).expect("Failed to read directory.");
        //TODO
    }
    true
}
fn check_head() -> bool {
    let head_dir = fs::read_dir(".riel/head").expect("Failed to read directory.");
    let head_files: Vec<fs::DirEntry> = head_dir.map(|x| x.unwrap()).collect::<Vec<fs::DirEntry>>();
    head_files.len() == 0
}
fn prepare_goto(hash: String) -> bool {
    let hash_reduced = hash;
    let dest_str = format!(".riel/commits/local/{}", hash_reduced);
    if !fs::metadata(&dest_str).is_ok() {
        println!(
            "Commit {} does not exist, check your hash or use riel load (Not developed yet)",
            hash_reduced
        ); //FIXME
        return false;
    }
    copy_recursive(Path::new(&dest_str), Path::new("."));
    true
}
