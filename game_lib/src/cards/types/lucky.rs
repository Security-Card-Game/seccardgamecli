use serde::{Deserialize, Serialize};
use crate::cards::properties::description::Description;
use crate::cards::properties::effect::Effect;
use crate::cards::properties::title::Title;
use crate::cards::types::card_model::Card;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LuckyCard {
    pub title: Title,
    pub description: Description,
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
