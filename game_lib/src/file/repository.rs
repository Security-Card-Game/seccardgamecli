use std::ffi::OsString;
use std::fs;
use std::path::PathBuf;
use std::rc::Rc;

use log::error;

use crate::cards::types::attack::AttackCard;
use crate::cards::types::card_model::{Card, CardTrait};
use crate::cards::types::event::EventCard;
use crate::cards::types::lucky::LuckyCard;
use crate::cards::types::oopsie::OopsieCard;
use crate::errors::{ErrorKind, GameLibError, GameLibResult};
use crate::file::cards::get_card_directory;
use crate::file::general::get_files_in_directory_with_filter;
use crate::world::deck::{CardRc, DeckRepository, GameVariantsRepository};

pub struct DeckLoader {
    base_path: String,
}

impl DeckRepository for DeckLoader {
    fn get_event_cards(&self) -> Vec<CardRc> {
        self.read_all_cards(&EventCard::empty())
    }

    fn get_lucky_cards(&self) -> Vec<CardRc> {
        self.read_all_cards(&LuckyCard::empty())
    }

    fn get_oopsie_cards(&self) -> Vec<CardRc> {
        self.read_all_cards(&OopsieCard::empty())
    }

    fn get_attack_cards(&self) -> Vec<CardRc> {
        self.read_all_cards(&AttackCard::empty())
    }
}

impl GameVariantsRepository for DeckLoader {
    fn get_scenarios(&self) -> Vec<Rc<crate::cards::game_variants::scenario::Scenario>> {
        let path = PathBuf::from(&self.base_path).join("scenarios");
        let scenarios_path = path.to_str().expect("Scenarios path");
        Self::load_cards(scenarios_path).unwrap_or_default()
    }
}

impl DeckLoader {
    pub fn create(base_path: &str) -> Self {
        DeckLoader {
            base_path: base_path.to_string(),
        }
    }

    fn read_all_cards(&self, card_type: &Card) -> Vec<Rc<Card>> {
        let path = PathBuf::from(&self.base_path).join(get_card_directory(card_type));
        let cards_path = path.to_str().expect("Card path");
        match Self::load_cards(cards_path) {
            Ok(cards) => cards,
            Err(err) => {
                error!(
                    "Could not read cards of type: {} (caused by {})",
                    card_type.category(),
                    err.to_string()
                );
                vec![]
            }
        }
    }

    fn deserialize_card<T>(file: OsString) -> GameLibResult<T> 
    where T: serde::de::DeserializeOwned
    {
        match fs::read_to_string(file) {
            Ok(content) => {
                let card = serde_json::from_str::<T>(content.as_str()).unwrap();
                Ok(card)
            }
            Err(err) => Err(GameLibError::create_with_original(
                ErrorKind::IO,
                "Could not deserialize card",
                err.to_string(),
            )),
        }
    }

    fn load_cards<T>(cards_path: &str) -> GameLibResult<Vec<Rc<T>>> 
    where T: serde::de::DeserializeOwned {
        let files = get_files_in_directory_with_filter(cards_path, ".json").map_err(|e| {
            GameLibError::create_with_original(ErrorKind::IO, "Could not read cards", e.to_string())
        })?;

        let mut cards : Vec<Rc<T>> = vec![];
        for file in files {
            let card = Self::deserialize_card(file)?;
            cards.push(Rc::new(card))
        }
        Ok(cards)
    }
}
