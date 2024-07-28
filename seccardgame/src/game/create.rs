use std::fs;
use std::path::PathBuf;

use dialoguer::Input;
use log::info;

use game_lib::cards::types::card_model::Card;
use game_lib::file::cards::write_data_to_file;
use game_lib::file::repository::DeckLoader;
use game_lib::world::deck::{Deck, DeckComposition, DeckPreparation, PreparedDeck};

use crate::cli::cli_result::{CliError, CliResult, ErrorKind};
use crate::cli::config::Config;

fn get_number_of_cards(prompt: &str) -> u8 {
    Input::new().with_prompt(prompt).interact().unwrap()
}

pub fn create_deck(config: &Config) -> Deck {
    let event_card_count = get_number_of_cards("Enter number of event types");
    let attack_card_count = get_number_of_cards("Enter number of attack types");
    let oopsie_card_count = get_number_of_cards("Enter number of oopsies");
    let lucky_card_count = get_number_of_cards("Enter number of lucky types");

    let deck_composition = DeckComposition {
        events: event_card_count as usize,
        attacks: attack_card_count as usize,
        oopsies: oopsie_card_count as usize,
        lucky: lucky_card_count as usize,
    };

    let prepared_deck = PreparedDeck::prepare(
        deck_composition,
        DeckLoader::create(config.game_path.as_str()),
    );

    prepared_deck.shuffle()
}

pub fn create_deck_and_write_to_disk(deck_path: String, config: &Config) -> CliResult<()> {
    let deck = create_deck(config);
    write_cards_to_deck(deck.remaining_cards, deck_path)?;

    info!("Deck created!");

    Ok(())
}

fn write_cards_to_deck(deck: Vec<Card>, path: String) -> CliResult<()> {
    fs::create_dir(&path).map_err(|e| CliError {
        kind: ErrorKind::FileSystemError,
        message: format!("Could not create directory {}", path).to_string(),
        original_message: Some(e.to_string()),
    })?;

    for (index, card) in deck.iter().enumerate() {
        let path_buff = PathBuf::from(&path);
        let path = path_buff.join(format!("{:0>3}.json", index));
        write_data_to_file(card, &path).expect("Data to have been written");
    }

    Ok(())
}
