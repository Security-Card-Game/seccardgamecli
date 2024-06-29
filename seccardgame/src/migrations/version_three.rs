use std::ffi::OsString;
use std::fs;
use std::path::{MAIN_SEPARATOR_STR, PathBuf};

use serde_json::{json, Number, Value};

use game_lib::cards::types::attack::AttackCard;
use game_lib::cards::types::card_model::{Card, CardTrait};
use game_lib::cards::types::event::EventCard;
use game_lib::cards::types::lucky::LuckyCard;
use game_lib::cards::types::oopsie::OopsieCard;
use game_lib::file::cards::{get_card_directory, write_data_to_file};
use game_lib::file::general::{ensure_directory_exists, get_files_in_directory_with_filter};

use crate::cli::cli_result::{CliError, CliResult, ErrorKind};
use crate::cli::config::Config;

pub fn convert(config: &Config) -> CliResult<()> {
    convert_cards(EventCard::empty(), &config.game_path);
    convert_cards(OopsieCard::empty(), &config.game_path);
    convert_cards(AttackCard::empty(), &config.game_path);
    convert_cards(LuckyCard::empty(), &config.game_path);

    Ok(())
}

fn convert_cards<T>(card_type: T, game_path: &String)
where
    T: CardTrait,
{
    const ATTACK_DIR: &'static str = "attacks";
    const INCIDENT_DIR: &'static str = "incidents";

    let binding = match card_type.as_enum() {
        Card::Attack(_) => PathBuf::from(game_path).join(INCIDENT_DIR),
        _ => PathBuf::from(game_path).join(get_card_directory(&card_type.as_enum())),
    };
    let card_path = binding.to_str().unwrap();
    let attack_path = PathBuf::from(game_path).join(ATTACK_DIR);

    ensure_directory_exists(attack_path.to_str().expect("A path"))
        .expect("Directory attack to be existing");
    let cards = get_files_in_directory_with_filter(card_path, ".json").unwrap();

    for card in cards.iter() {
        let content = fs::read_to_string(card)
            .map_err(|_| CliError {
                kind: ErrorKind::FileSystemError,
                message: "Could not read file".to_string(),
                original_message: None,
            })
            .unwrap();

        let mut v: Value = serde_json::from_str(content.as_str())
            .map_err(|e| CliError {
                kind: ErrorKind::CardError,
                message: "Could not parse into json".to_string(),
                original_message: Some(e.to_string()),
            })
            .unwrap();

        match card_type.as_enum() {
            Card::Attack(_) => {
                v["type"] = Value::String("attack".to_string());
            }
            _ => {}
        }

        if let Value::String(s) = v["action"].clone() {
            let mut map = serde_json::Map::new();
            match card_type.as_enum() {
                Card::Attack(_) => {
                    if let Value::Array(targets) = v["targets"].clone() {
                        let incident = json!([s.clone(), targets]);
                        map.insert("incident".to_string(), incident);
                    };
                }
                Card::Oopsie(_) => {
                    if let Value::Array(targets) = v["targets"].clone() {
                        let incident = json!([s.clone(), targets]);
                        map.insert("attackSurface".to_string(), incident);
                    };
                }
                _ => {
                    map.insert("other".to_string(), Value::String(s.clone()));
                }
            };
            v["effect"] = Value::Object(map);
        }

        if let Value::Number(n) = v["duration"].clone() {
            let mut map = serde_json::Map::new();
            map.insert("rounds".to_string(), Value::Number(n.clone()));
            v["duration"] = Value::Object(map);
        } else {
            let mut map = serde_json::Map::new();
            map.insert("rounds".to_string(), Value::Number(Number::from(3)));
            v["duration"] = Value::Object(map);
        }

        if let Value::Object(ref mut map) = v["fix_cost"].clone() {
            let min = &map["min"];
            let max = &map["max"];

            let mut map_fix = serde_json::Map::new();

            map_fix.insert("max".to_string(), max.clone());
            map_fix.insert("min".to_string(), min.clone());
            v["fixCost"] = Value::Object(map_fix.clone());
        }

        if let Some(map) = v.as_object_mut() {
            map.remove("action");
            map.remove("targets");
            map.remove("fix_cost");
        }

        let card_content: Card = match card_type.as_enum() {
            Card::Event(_) => Card::Event(serde_json::from_value::<EventCard>(v).unwrap()),
            Card::Attack(_) => Card::Attack(serde_json::from_value::<AttackCard>(v).unwrap()),
            Card::Oopsie(_) => Card::Oopsie(serde_json::from_value::<OopsieCard>(v).unwrap()),
            Card::Lucky(_) => Card::Lucky(serde_json::from_value::<LuckyCard>(v).unwrap()),
        };
        fs::remove_file(card).unwrap();

        let new_card_path = match card_type.as_enum() {
            Card::Attack(_) => {
                let incident_dir = INCIDENT_DIR.to_string() + &MAIN_SEPARATOR_STR;
                let attack_dir = ATTACK_DIR.to_string() + &MAIN_SEPARATOR_STR;
                let old_path = card.to_str().expect("Valid UTF-8 string");
                let new_path = old_path.replace(&incident_dir, &attack_dir);
                OsString::from(new_path)
            }
            _ => card.clone(),
        };

        write_data_to_file(&card_content, PathBuf::from(new_card_path).as_path())
            .map_err(|e| CliError {
                kind: ErrorKind::CardError,
                message: "Could not write to file".to_string(),
                original_message: Some(e.to_string()),
            })
            .unwrap();
    }
}
