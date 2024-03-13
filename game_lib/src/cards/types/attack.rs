use serde::{Deserialize, Serialize};

use crate::cards::properties::description::Description;
use crate::cards::properties::duration::Duration;
use crate::cards::properties::effect::Effect;
use crate::cards::properties::effect_description::EffectDescription;
use crate::cards::properties::target::Target;
use crate::cards::properties::title::Title;
use crate::cards::types::card_model::Card;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AttackCard {
    pub title: Title,
    pub description: Description,
    pub effect: Effect,
    pub duration: Duration,
}

impl AttackCard {
    pub fn new(
        title: Title,
        description: Description,
        targets: Vec<Target>,
        effect: EffectDescription,
        duration: Duration,
    ) -> Self {
        AttackCard {
            title,
            description,
            effect: Effect::Incident(effect, targets),
            duration,
        }
    }

    pub fn empty() -> Card {
        Card::Attack(AttackCard {
            title: Title::empty(),
            description: Description::empty(),
            effect: Effect::Incident(EffectDescription::empty(), vec![]),
            duration: Duration::default(),
        })
    }
}
