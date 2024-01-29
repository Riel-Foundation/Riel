use std::fs::{copy, create_dir_all, read_dir, File, ReadDir};
use std::io;

pub fn copy_recursive(src: &std::path::Path, dest: &std::path::Path) -> bool {
    if let Err(e) = copy_directory(src, dest) {
        eprintln!("Failed to copy directory: {}", e);
        return false;
    }

    true
}
pub fn copy_directory(src: &std::path::Path, dest: &std::path::Path) -> io::Result<()> {
    if src.is_dir() {
        create_dir_all(dest)?;

        for entry in read_dir(src)? {
            let entry = entry?;
            let entry_type = entry.file_type()?;

            let new_dest = dest.join(entry.file_name());

            if entry_type.is_dir() {
                copy_recursive(&entry.path(), &new_dest);
            } else {
                copy(&entry.path(), &new_dest)?;
            }
        }
    }

    Ok(())
}
pub fn read_dir_to_files(dir_result: Result<ReadDir, std::io::Error>) -> Result<Vec<File>, String> {
    let dir_entries = dir_result.map_err(|e| format!("Error reading directory: {}", e))?;

    let files: Vec<File> = dir_entries
        .filter_map(|entry| {
            if let Ok(entry) = entry {
                if entry.file_type().map_or(false, |ft| ft.is_file()) {
                    if let Ok(file) = File::open(entry.path()) {
                        return Some(file);
                    }
                }
            }
            None
        })
        .collect();

    Ok(files)
}
