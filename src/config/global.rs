use crate::args_parser::ParsedArgsObject;

pub fn start_config(args: ParsedArgsObject) {
  println!("Starting config");
  if args.subcommands().is_empty() {
    println!("No subcommands found");
  } else {
    println!("Subcommands found");
  }
}