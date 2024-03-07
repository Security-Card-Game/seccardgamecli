use std::ffi::OsString;
use std::fs;
use std::panic::panic_any;
use std::path::PathBuf;
use std::rc::Rc;

use dialoguer::Input;
use log::{info, warn};
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};

use game_lib::cards::types::attack::AttackCard;
use game_lib::cards::types::card_model::{Card, CardTrait};
use game_lib::cards::types::event::EventCard;
use game_lib::cards::types::lucky::LuckyCard;
use game_lib::cards::types::oopsie::OopsieCard;
use game_lib::cards::world::deck::EventCards::Lucky;
use game_lib::cards::world::deck::{
    DeckComposition, DeckPreparation, DeckRepository, EventCards, PreparedDeck, SharedCard,
};
use game_lib::file::cards::{get_card_directory, write_data_to_file};
use game_lib::file::general::{count_cards_in_directory, get_files_in_directory_with_filter};

use crate::cli::cli_result::{CliError, CliResult, ErrorKind};
use crate::cli::config::Config;

struct CardCounts {
    events: usize,
    incidents: usize,
    oopsies: usize,
    lucky: usize,
    total: usize,
}

struct DeckLoader {
    base_path: String,
}

impl DeckRepository for DeckLoader {
    fn get_event_cards(&self) -> Vec<SharedCard> {
        self.read_all_cards(&EventCard::empty())
    }

    fn get_lucky_cards(&self) -> Vec<SharedCard> {
        self.read_all_cards(&LuckyCard::empty())
    }

    fn get_oopsie_cards(&self) -> Vec<SharedCard> {
        self.read_all_cards(&OopsieCard::empty())
    }

    fn get_attack_cards(&self) -> Vec<SharedCard> {
        self.read_all_cards(&AttackCard::empty())
    }
}

impl DeckLoader {
    fn count_files(&self, card: &Card) -> u32 {
        let mut base_path = PathBuf::from(&self.base_path);
        let card_dir = get_card_directory(&card);
        base_path.push(card_dir);
        let path = base_path.to_str().unwrap().trim();
        count_cards_in_directory(path).unwrap_or_else(|e| {
            warn!("Error reading files for stats from {}: {}", path, e);
            0
        })
    }

    fn read_all_cards(&self, card_type: &Card) -> Vec<SharedCard> {
        let path = PathBuf::from(&self.base_path).join(get_card_directory(card_type));
        let cards_path = path.to_str().expect("Card path");
        Self::load_cards(cards_path)
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

    fn load_cards(cards_path: &str) -> Vec<SharedCard> {
        let files = get_files_in_directory_with_filter(cards_path, ".json")
            .map_err(|e| CliError {
                kind: ErrorKind::FileSystemError,
                message: "Could not read deck".to_string(),
                original_message: Some(e.to_string()),
            })
            .expect("Cards!!!");

        let mut cards = vec![];
        for file in files {
            match Self::deserialize_card(file) {
                Ok(card) => cards.push(Rc::new(card)),
                Err(err) => panic!("Could not read card!"),
            }
        }
        cards
    }
}

// Declaring a function to get user input
fn get_number_of_cards(prompt: &str) -> u8 {
    Input::new().with_prompt(prompt).interact().unwrap()
}

pub fn create_deck(deck_path: String, config: &Config) -> CliResult<()> {
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
        DeckLoader {
            base_path: config.game_path.to_string(),
        },
    );
    let deck = prepared_deck.shuffle();

    write_cards_to_deck(deck.cards, deck_path);

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
