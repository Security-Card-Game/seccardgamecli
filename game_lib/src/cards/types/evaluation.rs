use crate::cards::properties::description::Description;
use crate::cards::properties::effect::Effect;
use crate::cards::properties::title::Title;
use crate::cards::types::card_model::Card;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct EvaluationCard {
    pub title: Title,
    pub description: Description,
    pub effect: Effect,
}

impl Default for EvaluationCard {
    fn default() -> Self {
        EvaluationCard {
            title: Title::from("Evaluation".to_string()),
            description: Description::from("Be creative!".to_string()),
            effect: Effect::NOP,
        }
    }
}

impl EvaluationCard {
    pub fn empty() -> Card {
        Card::Evaluation(EvaluationCard {
            title: Title::empty(),
            description: Description::empty(),
            effect: Effect::NOP,
        })
    }

    pub fn is_closeable(&self) -> bool {
        true
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use fake::Dummy;
    use rand::Rng;

    use super::*;

    pub struct FakeEvaluationCard;

    impl Dummy<FakeEvaluationCard> for EvaluationCard {
        fn dummy_with_rng<R: Rng + ?Sized>(_: &FakeEvaluationCard, _: &mut R) -> Self {
            EvaluationCard::default()
        }
    }
}
