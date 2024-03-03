use serde::{Deserialize, Serialize};
use crate::cards::properties::description::Description;
use crate::cards::properties::duration::Duration;
use crate::cards::properties::effect::Effect;
use crate::cards::properties::target::Target;
use crate::cards::properties::title::Title;
use crate::cards::types::card_model::Card;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IncidentCard {
    pub title: Title,
    pub description: Description,
    pub targets: Vec<Target>,
    pub action: Effect,
    #[serde(default)]
    pub duration: Duration,
}

impl IncidentCard {
    pub fn empty() -> Card {
        Card::Incident(IncidentCard {
            title: Title::empty(),
            description: Description::empty(),
            targets: vec![],
            action: Effect::default(),
            duration: Duration::default(),
        })
    }
}
