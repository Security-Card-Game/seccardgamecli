use std::fs::File;
use std::io::Write;
use std::io::Read;
use crate::cards::model::Card;
use serde_json;

pub fn write_card_to_file(card: &Card) {
        let j = serde_json::to_string_pretty(&card).unwrap();
        let mut file = File::create("output.json").expect("Unable to create file");
        file.write_all(j.as_bytes()).expect("Unable_to_write_data");
        println!("Wrote to file output.json");
}

#[cfg(test)]
mod tests {
        use super::*;
        use std::fs;

        #[test]
        fn test_write_card_to_file() {
            let card = Card {
                    card_type: "type".to_string(),
                    title: "title".to_string(),
                    description: "description".to_string(),
                    cost: 55,
            };

            write_card_to_file(&card);

            let mut file = fs::File::open("output.json").expect("Unable to open file");
            let mut j = String::new();
            file.read_to_string(&mut j).expect("Unable to read file");

            assert_eq!(j, serde_json::to_string_pretty(&card).unwrap());

            fs::remove_file("output.json").unwrap();
        }
}