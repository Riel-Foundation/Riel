use std::env;
use std::fs;

fn main() {
    const RIEL_WORKS: &str = "Riel works! Try help or --help for more information";
    const HELP: &str = "Welcome to Riel! Try help or --help for more information, or init / create to start a repository.";
    let args: Vec<String> = env::args().collect();
    let fixed_args = fix_args(args);
    match fixed_args.len() { // NOTE: This structure is bound to change
        1 => println!
        ("{}", RIEL_WORKS),
        2 => match fixed_args[1].as_str() {
            "help" => println!("{}", HELP),
            "--help" => println!("{}", HELP),
            "init" => mount_repo(),
            "mount" => mount_repo(),
            _ => println!("{} is not a valid command. Try help or --help for more information.", fixed_args[1]),
        },
        _ => println!("Unrecognized. Try help or --help for more information"),
        }
    }
fn fix_args(args: Vec<String>) -> Vec<String> {
    let clean_args: Vec<String> = args.iter().map(|x| x.to_lowercase()).collect();
    clean_args.iter().map(|x| x.replace(" ", "")).collect()
}
fn mount_repo() -> () {
    if fs::metadata(".riel").is_ok() {
        println!("Riel repository already exists in this directory.");
    } else {
        create_repo();
        println!("Riel repository created successfully.");
    }

    fn create_repo() -> () {
        fs::create_dir(".riel").expect("Failed to create .riel directory, please check your permissions.");
        fs::create_dir(".riel/commits").expect("Failed to create .riel/objects directory, but .riel worked. Please check your storage & folder structure.");
    }
}