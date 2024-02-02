use dialoguer::Input;
use crate::cli::cli_result::{CliError, CliResult, ErrorKind};
use crate::cli::config::Config;

struct CardCounts {
    event: u8,
    incidents: u8,
    oopsies: u8,
    lucky: u8,
    total: u16,
}

// Declaring a function to get user input
fn get_number_of_cards(prompt: &str) -> u8 {
    Input::new()
        .with_prompt(prompt)
        .interact()
        .unwrap()
}

pub fn create_game(target_dir: String, config: &Config) -> CliResult<()> {
    let event_card_count = get_number_of_cards("Enter number of event cards");
    let incident_card_count = get_number_of_cards("Enter number of incident cards");
    let oopsie_card_count = get_number_of_cards("Enter number of oopsies");
    let lucky_card_count = get_number_of_cards("Enter number of lucky cards");

    let card_counts = CardCounts {
        event: event_card_count,
        incidents: incident_card_count,
        oopsies: oopsie_card_count,
        lucky: lucky_card_count,
        total: (event_card_count + incident_card_count + oopsie_card_count + lucky_card_count) as u16
    };

    println!("");
    println!("You are about to create a deck with {} cards in total.", card_counts.total);
    println!("It will contain {} events, {} oopsies, {} incidents and {} lucky happenings.", card_counts.event, card_counts.oopsies, card_counts.incidents, card_counts.lucky);
    println!("");


    Err(CliError {
        kind: ErrorKind::NotImplemented,
        message: "Not implemented".to_string(),
        original_message: None,
    })
}