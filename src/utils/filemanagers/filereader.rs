use std::fs::File;
use std::io::Read;
use std::path::Path;
pub fn open_path_and_read_to_string(path: &str) -> String {
    let p: &Path = Path::new(path);
    let mut file: File = File::open(p).expect(&format!(
        "Filereader.rs --> Could not open file while using path: {}",
        path
    ));
    let mut file_str: String = String::new();
    file.read_to_string(&mut file_str).expect(&format!(
        "Filereader.rs --> Could not read file while using path: {}",
        path
    ));

    file_str
}
