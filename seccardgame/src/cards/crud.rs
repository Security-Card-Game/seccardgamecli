use dialoguer::{Confirm, Select};
use game_lib::cards::model::{Card, EventCard};
use crate::cli::prompts::prompt;


fn write_card_to_file(card: &Card) {
    if Confirm::new()
        .with_prompt("Do you confirm these details?")
        .interact()
        .unwrap()
    {
        game_lib::file::cards::write_card_to_file(card)
    } else {
        println!("Cancelled!");
    }
}


pub fn create() {
    let card_type_index = Select::new()
        .items(&Card::CARD_TYPES)
        .default(0)
        .interact()
        .unwrap();

    let card = match Card::CARD_TYPES[card_type_index] {
        Card::EVENT_CARD => create_event_card(),
        _ => { todo!("Not yet done") }
    };

    write_card_to_file(&card);
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
        assert_eq!(card.action(), 10);
    }
}
