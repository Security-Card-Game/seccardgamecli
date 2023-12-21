use std::fs::File;
use std::io::Write;
use dialoguer::{Confirm, Input, Select};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Card {
    card_type: String,
    title: String,
    description: String,
    cost: u8,
}

const CARD_TYPES: &[&str; 3] = &["Magic", "Trap", "Monster"];


fn write_card_to_file(card: &Card) {
    if Confirm::new()
        .with_prompt("Do you confirm these details?")
        .interact()
        .unwrap()
    {
        let j = serde_json::to_string_pretty(&card).unwrap();
        let mut file = File::create("output.json").expect("Unable to create file");
        file.write_all(j.as_bytes()).expect("Unable_to_write_data");
        println!("Wrote to file output.json");
    } else {
        println!("Cancelled!");
    }
}

fn prompt<T: std::str::FromStr + Default>(
    prompt_msg: &str,
    validator: Option<Box<dyn Fn(&T) -> Result<(), String>>>,
) -> T
    where
        <T as std::str::FromStr>::Err: std::fmt::Debug + ToString, T: Clone + ToString
{
    let mut input = Input::<T>::new().with_prompt(prompt_msg);
    if let Some(validator) = validator {
        input = input.validate_with(validator);
    }
    input.interact().unwrap()
}

pub fn create() {
    let card_type_index = Select::new()
        .items(CARD_TYPES)
        .default(0)
        .interact()
        .unwrap();

    let title: String = prompt("Card title", None);
    let description: String = prompt("Card description", None);
    let cost: u8 = prompt("Costs", Some(Box::new(|input: &u8| -> Result<(), String> {
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