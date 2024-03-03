use std::fs;
use std::path::PathBuf;

use game_lib::cards::types::attack::IncidentCard;
use game_lib::cards::types::card_model::{Card, CardTrait};
use game_lib::cards::types::event::EventCard;
use game_lib::cards::types::lucky::LuckyCard;
use game_lib::cards::types::oopsie::OopsieCard;

use game_lib::file::cards::{get_card_directory, write_data_to_file};
use game_lib::file::general::get_files_in_directory_with_filter;

use crate::cli::cli_result::{CliError, CliResult, ErrorKind};
use crate::cli::config::Config;

pub fn convert(config: &Config) -> CliResult<()> {
    convert_cards(EventCard::empty(), &config.game_path);
    convert_cards(OopsieCard::empty(), &config.game_path);
    convert_cards(IncidentCard::empty(), &config.game_path);
    convert_cards(LuckyCard::empty(), &config.game_path);

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
        let card_content: Card = match card_type.as_enum() {
            Card::Event(_) => {
                Card::Event(serde_json::from_str::<EventCard>(content.as_str()).unwrap())
            }
            Card::Incident(_) => {
                Card::Incident(serde_json::from_str::<IncidentCard>(content.as_str()).unwrap())
            }
            Card::Oopsie(_) => {
                Card::Oopsie(serde_json::from_str::<OopsieCard>(content.as_str()).unwrap())
            }
            Card::Lucky(_) => {
                Card::Lucky(serde_json::from_str::<LuckyCard>(content.as_str()).unwrap())
            }
        };
        fs::remove_file(card).unwrap();
        write_data_to_file(&card_content, PathBuf::from(card).as_path())
            .map_err(|e| CliError {
                kind: ErrorKind::CardError,
                message: "Could not write to file".to_string(),
                original_message: Some(e.to_string()),
            })
            .unwrap();
    }
}
