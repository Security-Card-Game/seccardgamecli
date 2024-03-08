use crate::cards::types::attack::AttackCard;
use crate::cards::types::card_model::{Card, CardTrait};
use crate::cards::types::event::EventCard;
use crate::cards::types::lucky::LuckyCard;
use crate::cards::types::oopsie::OopsieCard;
use crate::cards::world::deck::{CardRc, DeckRepository};
use crate::errors::{ErrorKind, GameLibError, GameLibResult};
use crate::file::cards::get_card_directory;
use crate::file::general::{count_cards_in_directory, get_files_in_directory_with_filter};
use log::{error, warn};
use std::ffi::OsString;
use std::fs;
use std::path::PathBuf;
use std::rc::Rc;

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

impl DeckLoader {

    pub fn create(base_path: &str) -> Self {
        DeckLoader {
            base_path: base_path.to_string()
        }
    }

    fn read_all_cards(&self, card_type: &Card) -> Vec<CardRc> {
        let path = PathBuf::from(&self.base_path).join(get_card_directory(card_type));
        let cards_path = path.to_str().expect("Card path");
        match Self::load_cards(cards_path) {
            Ok(cards) => cards,
            Err(err) =>  {
                error!("Could not read cars of type: {} (caused by {})", card_type.category(), err.to_string());
                vec![]
            }
        }
    }

    fn deserialize_card(file: OsString) -> GameLibResult<Card> {
        match fs::read_to_string(file) {
            Ok(content) => {
                let card = serde_json::from_str::<Card>(content.as_str()).unwrap();
                Ok(card)
            }
            Err(err) => Err(GameLibError::create_with_original(
                ErrorKind::IO,
                "Could not deserialize card",
                err.to_string(),
            )),
        }
    }

    fn load_cards(cards_path: &str) -> GameLibResult<Vec<CardRc>> {
        let files = get_files_in_directory_with_filter(cards_path, ".json")
            .map_err(|e| {
                GameLibError::create_with_original(
                    ErrorKind::IO,
                    "Could not read cards",
                    e.to_string(),
                )
            })?;

        let mut cards = vec![];
        for file in files {
            let card = Self::deserialize_card(file)?;
            cards.push(Rc::new(card))
        }
        Ok(cards)
    }
}
