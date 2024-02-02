use dialoguer::{Confirm, Editor, Select};
use log::error;

use game_lib::cards::model::{Card, EventCard, FixCost, IncidentCard, LuckyCard, OopsieCard};

use crate::cards::stats::print_stats;
use crate::cli::cli_result::ErrorKind::FileSystemError;
use crate::cli::cli_result::{CliError, CliResult, ErrorKind};
use crate::cli::config::Config;
use crate::cli::prompts::{prompt, prompt_allow_empty};

fn write_card_to_file(card: &Card, cfg: &Config) -> CliResult<()> {
    let mut card_to_save: Card = card.clone();
    if Confirm::new()
        .with_prompt("Do you want to edit this card?")
        .interact()
        .unwrap()
    {
        card_to_save =
            match Editor::new().edit(serde_json::to_string_pretty(card).unwrap().as_str()) {
                Ok(content) => match content {
                    Some(edited) => deserialize_editor_content(edited, &card).unwrap_or_else(|e| {
                        error!("Error deserializing into card: {}", e);
                        card_to_save
                    }),
                    None => card_to_save,
                },
                Err(e) => {
                    return Err(CliError::new(
                        ErrorKind::CardError,
                        "Could not edit card",
                        Some(e.to_string()),
                    ))
                }
            };
        println!(
            "Card to save is\n{}",
            serde_json::to_string_pretty(&card_to_save).unwrap()
        );
    }
    if Confirm::new()
        .with_prompt("Do you confirm these details?")
        .interact()
        .unwrap()
    {
        game_lib::file::cards::write_card_to_file(&card_to_save, Some(cfg.game_path.as_str()))
            .map_err(|e| {
                CliError::new(
                    FileSystemError,
                    "Could not write to file!",
                    Some(e.to_string()),
                )
            })?;
    } else {
        println!("Cancelled!");
    }
    Ok(())
}

fn deserialize_editor_content(content: String, original_card: &Card) -> serde_json::Result<Card> {
    match original_card {
        Card::Event(_) => serde_json::from_str(content.as_str()).map(|c| Card::Event(c)),
        Card::Incident(_) => serde_json::from_str(content.as_str()).map(|c| Card::Incident(c)),
        Card::Oopsie(_) => serde_json::from_str(content.as_str()).map(|c| Card::Oopsie(c)),
        Card::Lucky(_) => serde_json::from_str(content.as_str()).map(|c| Card::Lucky(c)),
    }
}

pub fn create(cfg: &Config) -> CliResult<()> {
    print_stats(cfg)?;
    println!();
    let card_type_index = Select::new()
        .with_prompt("Select a card type to create")
        .items(&Card::CARD_TYPES)
        .default(0)
        .interact()
        .unwrap();

    let card = match Card::CARD_TYPES[card_type_index] {
        Card::EVENT_CARD => create_event_card(),
        Card::INCIDENT_CARD => create_incident_card(),
        Card::LUCKY_CARD => create_lucky_card(),
        Card::OOPSIE_CARD => create_oopsie_card(),
        _ => {
            return Err(CliError::new(
                ErrorKind::CardError,
                "Unknown card type",
                None,
            ))
        }
    };

    write_card_to_file(&card, cfg)
}

fn create_event_card() -> Card {
    println!("Create a new Event Card");
    let title: String = prompt("Card title", None);
    let description: String = prompt("Card description", None);
    let action: String = prompt("Card Action", None);

    let card = EventCard {
        title,
        description,
        action,
    };

    println!("{}", serde_json::to_string_pretty(&card).unwrap());

    Card::Event(card)
}

fn create_incident_card() -> Card {
    println!("Create a new Incident Card");
    let title: String = prompt("Card title", None);
    let description: String = prompt("Card description", None);
    let action: String = prompt("Card Action", None);

    let targets = ask_for_targets();

    let card = IncidentCard {
        title,
        description,
        action,
        targets,
    };

    println!("{}", serde_json::to_string_pretty(&card).unwrap());

    Card::Incident(card)
}

fn ask_for_targets() -> Vec<String> {
    println!("Add targets of this card, enter a blank target when finished");
    let mut targets: Vec<String> = Vec::new();

    loop {
        let target: String = prompt_allow_empty("Add incident target");
        if target.is_empty() {
            break;
        }
        targets.push(target);
    }
    targets
}

fn create_lucky_card() -> Card {
    println!("Create a new Lucky Card");
    let title: String = prompt("Card title", None);
    let description: String = prompt("Card description", None);
    let action: String = prompt("Card Action", None);

    let card = LuckyCard {
        title,
        description,
        action,
    };

    println!("{}", serde_json::to_string_pretty(&card).unwrap());

    Card::Lucky(card)
}

fn create_oopsie_card() -> Card {
    println!("Create a new Oopsie Card");
    let title: String = prompt("Card title", None);
    let description: String = prompt("Card description", None);
    let action: String = prompt("Card Action", None);
    let targets = ask_for_targets();
    let mut min_cost: u8;
    let mut max_cost: u8;
    loop {
        min_cost = prompt("Minimal fixing costs", None);
        max_cost = prompt("Maximal fixing costs", None);
        if min_cost <= max_cost {
            break;
        }
        println!("Max cost must be greater or equal to min cost.")
    }
    let card = OopsieCard {
        title,
        description,
        action,
        targets,
        fix_cost: FixCost {
            min: min_cost,
            max: max_cost,
        },
    };

    println!("{}", serde_json::to_string_pretty(&card).unwrap());

    Card::Oopsie(card)
}

#[cfg(test)]
mod tests {
    use game_lib::cards::model::CardTrait;

    use super::*;

    #[test]
    fn test_card_creation() {
        let card = Card::Event(EventCard {
            title: "Magic Card".to_string(),
            description: "This is a magical card.".to_string(),
            action: "10".to_string(),
        });

        assert_eq!(card.title(), "Magic Card");
        assert_eq!(card.description(), "This is a magical card.");
        assert_eq!(card.action(), "10");
    }
}
