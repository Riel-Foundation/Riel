use super::rielignore::{get_ignored, Ignores};
use std::fs;
pub fn add_files(subcommands: Vec<String>, options: Vec<String>) -> bool {
  //FIXME: Add a check to see if the files are already in the area
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