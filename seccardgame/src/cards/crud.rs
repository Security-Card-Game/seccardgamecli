use dialoguer::{Confirm, Select};
use game_lib::cards::model::{Card, CARD_TYPES};
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
        .items(CARD_TYPES)
        .default(0)
        .interact()
        .unwrap();

    let title: String = prompt("Card title", None);
    let description: String = prompt("Card description", None);
    let cost: u8 = prompt("Costs (0-20)", Some(Box::new(|input: &u8| -> Result<(), String> {
        if *input <= 20 { Ok(()) } else { Err(String::from("Costs should be between 0 and 20")) }
    })));

    let card = Card {
        card_type: CARD_TYPES[card_type_index].to_string(),
        title,
        description,
        cost,
    };

    println!("{:?}", card);
    write_card_to_file(&card);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_creation() {
        let card = Card {
            card_type: "Magic".to_string(),
            title: "Magic Card".to_string(),
            description: "This is a magical card.".to_string(),
            cost: 10,
        };

        assert_eq!(card.card_type, "Magic");
        assert_eq!(card.title, "Magic Card");
        assert_eq!(card.description, "This is a magical card.");
        assert_eq!(card.cost, 10);
    }
}
