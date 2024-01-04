use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    
    if args.len() > 1 {
        if args[1] == "--help" {
            println!("usage: riel <command> [<args>]");
        } else {
            println!("Welcome to Riel! Try using the --help flag for more information.");
        }
    } else {
        println!("Welcome to Riel! Try using the --help flag for more information.");
    }
}
