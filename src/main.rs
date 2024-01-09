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
            if add_file(subcommands) {
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
fn add_file(subcommands: Vec<String>) -> bool {
    if !check_repo() {
        println!("No repository found. Try init or mount.");
        return false;
    }
    match subcommands.len() {
        0 => println!("No files specified."),
        1.. => {
            for file in subcommands {
                if fs::metadata(&file).is_ok() {
                    fs::copy(&file, format!(".riel/area/{}", &file)).expect("Failed to copy file.");
                    println!("Prepared {}.", file);
                } else {
                    println!("{} does not exist.", file);
                    return false;
                }
            }
        },
        _ => println!("Failed to parse command."),
    }
    true
}
fn check_repo() -> bool {
    fs::metadata(".riel").is_ok()
}