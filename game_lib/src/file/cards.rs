use std::{fs, io};
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use crate::cards::model::{Card, CardTrait};
use serde_json;

pub fn write_card_to_file(card: &Card) -> std::io::Result<()> {
    let card_directory = get_card_directory(card);
    match ensure_directory_exists(&card_directory) {
        Ok(_) => (),
        Err(_) => return Err(io::Error::new(io::ErrorKind::NotFound, format!("Could not create directory {}", card_directory))),
    }
    let current_cards_count = match count_files_in_directory(&card_directory) {
        Ok(card_number) => card_number,
        Err(_) => return Err(io::Error::new(io::ErrorKind::NotFound, "Could not count files in directory")),
    };

    let file_name = generate_filename(card.title(), current_cards_count);
    let mut path = PathBuf::from(card_directory);
    path.push(file_name);

    write_data_to_file(card, &path)?;

    println!("Wrote to file {}", path.display());
    Ok(())
}

fn generate_filename(title: &str, current_cards_count: u32) -> String {
    let new_card_number = current_cards_count + 1;
    let padded_prefix = format!("{:0>4}", new_card_number);
    let sanitized_title = sanitize_filename(title);
    let shortened_title: String = sanitized_title.chars().take(60).collect();
    format!("{}-{}.json", padded_prefix, shortened_title)
}

fn write_data_to_file(card: &Card, path: &Path) -> std::io::Result<()> {
    match serde_json::to_string_pretty(&card) {
        Ok(serialized_card) => {
            let mut file = File::create(&path)?;
            file.write_all(serialized_card.as_bytes())
        },
        Err(_) => Err(io::Error::new(io::ErrorKind::Other, "Unable to serialize data")),
    }
}


fn get_card_directory(card: &Card) -> &'static str {
    match card {
        Card::Event(_) => "events",
        Card::Incident(_) => "incidents",
        Card::Oopsie(_) => "oppsies",
        Card::Lucky(_) => "lucky",
    }
}

fn ensure_directory_exists(path: &str) -> std::io::Result<()> {
    if !Path::new(path).exists() {
        fs::create_dir_all(path)?; // create_dir_all also creates the parent directories if they don't exist
    }
    Ok(())
}


fn count_files_in_directory(dir: &str) -> io::Result<u32> {
    let mut file_count = 0;

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        if entry.metadata()?.is_file() {
            file_count += 1;
        }
    }

    Ok(file_count)
}

fn replace_invalid_character(c: char) -> char {
    match c.is_alphanumeric() || c == '.' || c == '-' {
        true => c,
        false => '_',
    }
}

fn sanitize_filename(filename: &str) -> String {
    filename.chars().map(replace_invalid_character).collect()
}



