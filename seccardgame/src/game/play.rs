use std::ffi::OsString;
use std::fs;

use game_lib::cards::card_model::Card;
use game_lib::file::general::get_files_in_directory_with_filter;

use crate::cli::cli_result::{CliError, CliResult, ErrorKind};

pub fn play_deck(deck_path: String) -> CliResult<()> {
    let deck = load_cards(deck_path)?;

    game_ui::start::run(deck).map_err(|e| CliError {
        kind: ErrorKind::GUI,
        message: "Could not open GUI".to_string(),
        original_message: Some(e.to_string()),
    })
}

fn load_cards(deck_path: String) -> CliResult<Vec<Card>> {
    let files = get_files_in_directory_with_filter(&deck_path, ".json").map_err(|e| CliError {
        kind: ErrorKind::FileSystemError,
        message: "Could not read deck".to_string(),
        original_message: Some(e.to_string()),
    })?;

    let mut cards = vec![];
    for file in files {
        match deserialize_card(file) {
            Ok(card) => cards.push(card),
            Err(err) => return Err(err),
        }
    }
    Ok(cards)
}

fn deserialize_card(file: OsString) -> CliResult<Card> {
    match fs::read_to_string(file) {
        Ok(content) => {
            let card = serde_json::from_str::<Card>(content.as_str()).unwrap();
            Ok(card)
        }
        Err(err) => Err(CliError {
            kind: ErrorKind::FileSystemError,
            message: "Could not read file".to_string(),
            original_message: Some(err.to_string()),
        }),
    }
}
