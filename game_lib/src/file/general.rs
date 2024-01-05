use std::path::Path;
use std::{fs, io};

pub fn ensure_directory_exists(path: &str) -> std::io::Result<()> {
    if !Path::new(path).exists() {
        fs::create_dir_all(path)?; // create_dir_all also creates the parent directories if they don't exist
    }
    Ok(())
}

pub fn count_cards_in_directory(dir: &str) -> std::io::Result<u32> {
    count_files_in_directory_with_filter(dir, ".json")
}

pub fn count_files_in_directory_with_filter(dir: &str, filter: &str) -> io::Result<u32> {
    let mut file_count = 0;

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        if entry.metadata()?.is_file() && entry.file_name().to_str().unwrap().contains(filter) {
            file_count += 1;
        }
    }

    Ok(file_count)
}
