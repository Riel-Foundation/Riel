#![allow(unused_variables)]
use std::env;
use std::fs;

fn main() {
    const RIEL_WORKS: &str = "Riel works! Try help or --help for more information";
    const HELP: &str = "Welcome to Riel! Try help or --help for more information, or init / create to start a repository.";
    let args: Vec<String> = env::args().collect();
    let fixed_args = fix_args(args.clone());
    // let riel = fixed_args[0].as_str();
    let command: &str = fixed_args[1].as_str();
    let command_args = args[2..].to_vec();
    match fixed_args.len() { // NOTE: This structure is bound to change
        1 => println!
        ("{}", RIEL_WORKS),
        2.. => match command {
            "help" => println!("{}", HELP),
            "--help" => println!("{}", HELP),
            "init" => exec(command, command_args),
            "mount" => exec("init", command_args),
            "commit" => exec(command, command_args),
            "add" => exec(command, command_args),
            _ => println!("{} is not a valid command. Try help or --help for more information.", command),
        },
        _ => println!("Failed to parse command."),
        }
    }
fn fix_args(args: Vec<String>) -> Vec<String> {
    let clean_args: Vec<String> = args.iter().map(|x| x.to_lowercase()).collect();
    clean_args.iter().map(|x| x.replace(" ", "")).collect()
}
fn exec(command: &str, subcommands: Vec<String>) -> () {
    match command {
        "init" => mount_repo(), // for now, no subcommands,
        "commit" => {
            if commit(subcommands) {
                println!("Commited.");
            } else {
                println!("Commit failed.");
            }},
        "add" =>  {
            if add_files(subcommands) {
                println!("Added file(s).");
            } else {
                println!("Could not add all the files.");
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

    fn create_repo() -> () {
        fs::create_dir(".riel").expect("Failed to create .riel directory, please check your permissions.");
        fs::create_dir(".riel/commits").expect("Failed to create .riel/commits directory, but .riel worked. Please check your storage & folder structure.");
        fs::create_dir(".riel/area").expect("Failed to create .riel/area directory, but .riel worked. Please check your storage & folder structure.");
    }
}
fn commit(commit_args: Vec<String>) -> bool {
   /* let lower_args: Vec<String> = commit_args.iter().map(|x| x.to_lowercase()).collect();
    let args: Vec<&str> = lower_args.iter().map(|x| x.as_str()).collect::<Vec<&str>>();
    if args.contains(&"-all") || args.contains(&"-A") {
    }else {
    }
    false*/ 
    todo!()
}
fn add_files(subcommands: Vec<String>) -> bool {
    if !check_repo() {
        println!("No valid Riel repository found. Try init or mount.");
        return false;
    }
    let checking_vec = subcommands.iter().map(|x| x.as_str()).collect::<Vec<&str>>();
    let should_commit_all: bool = checking_vec.contains(&"-all") || checking_vec.contains(&"-A");
    match subcommands.len() {
        0 => println!("No files specified."),
        1.. => {
            if should_commit_all {
                let ignored: Ignores = get_ignored();
                if ignored.exists {
                    // should add all files except ignored and .riel
                    let ignore_list: Vec<String> = ignored.ignored.iter().map(|x| x.to_string()).collect();
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
    ignored: Vec<String>,
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
        ignored,
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