use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let clean_args: Vec<String> = args.iter().map(|x| x.to_lowercase()).collect();
    let fixed_args: Vec<String> = clean_args.iter().map(|x| x.replace(" ", "")).collect();
    match fixed_args.len() { // NOTE: This structure is bound to change
        0 => println!
        ("Welcome to Riel! Try help or --help for more information, or init / create to start a repository."),
        1 => match args[0].as_str() {
            "help" => println!("Welcome to Riel! Try help or --help for more information, or init / create to start a repository."),
            "--help" => println!("Welcome to Riel! Try help or --help for more information, or init / create to start a repository."),
            "init" => println!("Initializing repository..."), //TODO
            "create" => println!("Creating repository..."), // TODO
            _ => println!("Welcome to Riel! Try help or --help for more information, or init / create to start a repository."),
        },
        _ => println!("Welcome to Riel! Try help or --help for more information, or init / create to start a repository."),
        }
    }
