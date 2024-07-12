use fake::Dummy;
use serde::{Deserialize, Serialize};
use fake::faker::lorem::en::{ Words, Sentences};

use crate::cards::properties::description::Description;
use crate::cards::properties::effect::Effect;
use crate::cards::properties::title::Title;
use crate::cards::types::card_model::Card;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[derive(Dummy)]
#[serde(rename_all = "camelCase")]
pub struct LuckyCard {
    #[dummy(faker = "Words(3)")]
    pub title: Title,
    #[dummy(faker = "Sentences(3)")]
    pub description: Description,
    #[dummy(default)]
    pub effect: Effect,
}

impl LuckyCard {
    pub fn empty() -> Card {
        Card::Lucky(LuckyCard {
            title: Title::empty(),
            description: Description::empty(),
            effect: Effect::default(),
        })
    }
}
