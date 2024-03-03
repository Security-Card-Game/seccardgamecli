use std::fs;
use std::path::PathBuf;
use game_lib::cards::properties::duration::Duration;
use game_lib::cards::types::attack::IncidentCard;
use game_lib::cards::types::card_model::{Card, CardTrait};

use game_lib::file::cards::{get_card_directory, write_data_to_file};
use game_lib::file::general::get_files_in_directory_with_filter;

use crate::cli::cli_result::{CliError, CliResult, ErrorKind};
use crate::cli::config::Config;

pub fn convert(config: &Config) -> CliResult<()> {
    convert_cards(IncidentCard::empty(), &config.game_path);

    Ok(())
}

fn convert_cards<T>(card_type: T, game_path: &String)
where
    T: CardTrait,
{
    let binding = PathBuf::from(game_path).join(get_card_directory(&card_type.as_enum()));
    let card_path = binding.to_str().unwrap();
    let cards = get_files_in_directory_with_filter(card_path, ".json").unwrap();
    for card in cards.iter() {
        let content = fs::read_to_string(card)
            .map_err(|_| CliError {
                kind: ErrorKind::FileSystemError,
                message: "Could not read file".to_string(),
                original_message: None,
            })
            .unwrap();
        let old_card: IncidentCard =
            serde_json::from_str::<IncidentCard>(content.as_str()).unwrap();
        let new_card: Card = Card::Incident(IncidentCard {
            title: old_card.title,
            description: old_card.description,
            targets: old_card.targets,
            action: old_card.action,
            duration: Duration::Rounds(3),
        });
        fs::remove_file(card).unwrap();
        write_data_to_file(&new_card, PathBuf::from(card).as_path())
            .map_err(|e| CliError {
                kind: ErrorKind::CardError,
                message: "Could not write to file".to_string(),
                original_message: Some(e.to_string()),
            })
            .unwrap();
    }
}
