use crate::cli::prompts::{prompt, prompt_allow_empty};
use dialoguer::{Confirm, Select};
use game_lib::cards::model::{Card, EventCard, FixCost, IncidentCard, LuckyCard, OopsieCard};
use game_lib::print_to_stderr;
use crate::cli::config::Config;

fn write_card_to_file(card: &Card, cfg: &Config) {
    if Confirm::new()
        .with_prompt("Do you confirm these details?")
        .interact()
        .unwrap()
    {
        match game_lib::file::cards::write_card_to_file(card, Some(cfg.game_path.as_str())) {
            Ok(_) => (),
            Err(e) => print_to_stderr(format!("Could not write file\n {}", e.to_string()).as_str()),
        }
    } else {
        println!("Cancelled!");
    }
}

pub fn create(cfg: &Config) {
    let card_type_index = Select::new()
        .items(&Card::CARD_TYPES)
        .default(0)
        .interact()
        .unwrap();

    let card = match Card::CARD_TYPES[card_type_index] {
        Card::EVENT_CARD => create_event_card(),
        Card::INCIDENT_CARD => create_incident_card(),
        Card::LUCKY_CARD => create_lucky_card(),
        Card::OOPSIE_CARD => create_oopsie_card(),
        _ => panic!("Unknown card type!"),
    };

    write_card_to_file(&card, cfg);
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
    use super::*;
    use game_lib::cards::model::CardTrait;

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
