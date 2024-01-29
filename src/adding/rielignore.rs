use std::fs;
pub struct Ignores {
    pub exists: bool,
    pub files: Vec<String>,
}
pub fn get_ignored() -> Ignores {
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
