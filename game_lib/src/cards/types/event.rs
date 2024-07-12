use fake::{ Dummy, Fake };
use fake::faker::lorem::en::{ Words, Sentences};
use serde::{Deserialize, Serialize};

use crate::cards::properties::description::Description;
use crate::cards::properties::effect::Effect;
use crate::cards::properties::title::Title;
use crate::cards::types::card_model::Card;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[derive(Dummy)]
#[serde(rename_all = "camelCase")]
pub struct EventCard {
    #[dummy(faker = "Words(3..3)")]
    pub title: Title,
    #[dummy(faker = "Sentences(2..4)")]
    pub description: Description,
    #[dummy(default)]
    pub effect: Effect,
}

impl EventCard {
    pub fn empty() -> Card {
        Card::Event(EventCard {
            title: Title::empty(),
            description: Description::empty(),
            effect: Effect::NOP,
        })
    }
}
