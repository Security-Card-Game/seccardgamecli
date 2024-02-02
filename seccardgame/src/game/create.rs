use std::error::Error;
use std::ffi::{OsStr, OsString};
use std::fs;
use std::path::PathBuf;
use dialoguer::Input;
use rand::{Rng, thread_rng};
use rand::seq::SliceRandom;
use game_lib::cards::model::{Card, CardTrait, EventCard, IncidentCard, LuckyCard, OopsieCard};
use game_lib::cards::model::Card::Oopsie;
use game_lib::file::cards::get_card_directory;
use game_lib::file::general::{count_files_in_directory_with_filter, get_files_in_directory_with_filter};
use crate::cli::cli_result::{CliError, CliResult, ErrorKind};
use crate::cli::config::Config;

struct CardCounts {
    events: usize,
    incidents: usize,
    oopsies: usize,
    lucky: usize,
    total: usize,
}

// Declaring a function to get user input
fn get_number_of_cards(prompt: &str) -> u8 {
    Input::new()
        .with_prompt(prompt)
        .interact()
        .unwrap()
}

pub fn create_game(deck_path: String, config: &Config) -> CliResult<()> {
    let event_card_count = get_number_of_cards("Enter number of event cards");
    let incident_card_count = get_number_of_cards("Enter number of incident cards");
    let oopsie_card_count = get_number_of_cards("Enter number of oopsies");
    let lucky_card_count = get_number_of_cards("Enter number of lucky cards");

    let card_counts = CardCounts {
        events: event_card_count as usize,
        incidents: incident_card_count as usize,
        oopsies: oopsie_card_count as usize,
        lucky: lucky_card_count as usize,
        total: (event_card_count + incident_card_count + oopsie_card_count + lucky_card_count) as usize
    };

    let deck = draw_cards(&config, card_counts)?;

    write_cards_to_deck(deck, deck_path)
}

fn write_cards_to_deck(deck: Vec<OsString>, path: String) ->  CliResult<()>{
    fs::create_dir(&path).map_err(|e| CliError {
        kind: ErrorKind::FileSystemError,
        message: format!("Could not create directory {}", path).to_string(),
        original_message: Some(e.to_string()),
    })?;
    for (index, card) in deck.iter().enumerate() {
        let mut path_buff = PathBuf::from(&path);
        fs::copy(card, path_buff.join(format!("{:03}.json", index)))
            .map_err(|e| CliError {
                kind: ErrorKind::FileSystemError,
                message: "Could not copy card".to_string(),
                original_message: Some(e.to_string()),
            })?;
    }

    log::info!("Card copied to {}", path);

    Ok(())
}

fn draw_cards(config: &&Config, card_counts: CardCounts) -> Result<Vec<OsString>, CliError> {
    log::info!("Drawing cards");

    println!();
    println!("You are about to create a deck with {} cards in total.", card_counts.total);
    println!("It will contain {} events, {} oopsies, {} incidents and {} lucky happenings.", card_counts.events, card_counts.oopsies, card_counts.incidents, card_counts.lucky);
    println!();


    let event_cards = get_cards(EventCard::empty(), card_counts.events, &config.game_path)?;
    let incident_cards = get_cards(IncidentCard::empty(), card_counts.incidents, &config.game_path)?;
    let oopsie_cards = get_cards(OopsieCard::empty(), card_counts.oopsies, &config.game_path)?;
    let lucky_cards = get_cards(LuckyCard::empty(), card_counts.lucky, &config.game_path)?;

    let deck = create_deck(card_counts, event_cards, incident_cards, oopsie_cards, lucky_cards);

    log::info!("Cards are drawn and shuffled!");
    Ok(deck)
}

fn create_deck(card_counts: CardCounts, event_cards: Vec<OsString>, incident_cards: Vec<OsString>, oopsie_cards: Vec<OsString>, lucky_cards: Vec<OsString>) -> Vec<OsString> {
    let mut deck = vec![];

    deck.extend(event_cards);
    deck.extend(oopsie_cards);
    deck.extend(lucky_cards);

    let mut rng = thread_rng();
    deck.shuffle(&mut rng);

    let (safe_part, unsafe_part) = deck.split_at((card_counts.total / 4));

    let mut part_with_incidents = unsafe_part.to_vec();
    part_with_incidents.extend(incident_cards);
    part_with_incidents.shuffle(&mut rng);

    deck = safe_part.to_vec();
    deck.extend(part_with_incidents);
    deck
}

fn get_cards(card_type: Card, card_count: usize, game_path: &String) -> CliResult<Vec<OsString>> {
    dbg!("Getting cards for {}", card_type.category());
    let card_path = get_card_directory(&card_type);
    let mut path = PathBuf::from(game_path);
    path.push(card_path);

    let cards_total = get_files_in_directory_with_filter(path.to_str().unwrap(), ".json")
        .map_err(|e| CliError {
            kind: ErrorKind::FileSystemError,
            message: "Could not read card directory".to_string(),
            original_message: Some(e.to_string()),
        })?;

    log::info!("Found {} {} cards", cards_total.len(), card_type.category().to_lowercase());

    let mut x = 0;
    let mut cards = vec![];
    while (x < card_count) {
        let card_to_include = rand::thread_rng().gen_range(0..cards_total.len());
        cards.push(cards_total[card_to_include].clone());
        x += 1;
    }
    Ok(cards)
}