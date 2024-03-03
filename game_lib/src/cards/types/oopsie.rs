use serde::{Deserialize, Serialize};
use crate::cards::properties::description::Description;
use crate::cards::properties::effect::Effect;
use crate::cards::properties::fix_cost::FixCost;
use crate::cards::properties::target::Target;
use crate::cards::properties::title::Title;
use crate::cards::types::card_model::Card;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OopsieCard {
    pub title: Title,
    pub description: Description,
    pub targets: Vec<Target>,
    pub effect: Effect,
    pub fix_cost: FixCost,
}

impl OopsieCard {
    pub fn empty() -> Card {
        Card::Oopsie(OopsieCard {
            title: Title::empty(),
            description: Description::empty(),
            targets: vec![],
            effect: Effect::default(),
            fix_cost: FixCost::default(),
        })
    }
}
