use std::path::Path;
use std::{fs, io};

pub fn ensure_directory_exists(path: &str) -> std::io::Result<()> {
    if !Path::new(path).exists() {
        fs::create_dir_all(path)?; // create_dir_all also creates the parent directories if they don't exist
    }
    Ok(())
}

pub fn count_files_in_directory(dir: &str) -> io::Result<u32> {
    let mut file_count = 0;

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        if entry.metadata()?.is_file() {
            file_count += 1;
        }
    }

    Ok(file_count)
}
