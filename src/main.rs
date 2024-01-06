use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let fixed_args = fix_args(args);
    match fixed_args.len() { // NOTE: This structure is bound to change
        0 => println!
        ("Welcome to Riel! Try help or --help for more information, or init / create to start a repository."),
        1 => match fixed_args[0].as_str() {
            "help" => println!("Welcome to Riel! Try help or --help for more information, or init / create to start a repository."),
            "--help" => println!("Welcome to Riel! Try help or --help for more information, or init / create to start a repository."),
            "init" => mount_repo(),
            "mount" => mount_repo(),
            _ => println!("Welcome to Riel! Try help or --help for more information, or init / create to start a repository."),
        },
        _ => println!("Welcome to Riel! Try help or --help for more information, or init / create to start a repository."),
        }
    }
fn fix_args(args: Vec<String>) -> Vec<String> {
    let clean_args: Vec<String> = args.iter().map(|x| x.to_lowercase()).collect();
    clean_args.iter().map(|x| x.replace(" ", "")).collect()
}
fn mount_repo() -> () {
   // Create a .riel folder
   fs::create_dir(".riel").expect("Failed to create .riel folder, please check your permissions.");
    // Create a .riel/objects folder
    fs::create_dir(".riel/commits").expect("Failed to create .riel/objects folder, please check your permissions.");
}