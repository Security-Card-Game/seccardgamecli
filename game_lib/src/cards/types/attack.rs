use fake::{Dummy, Fake};
use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::cards::properties::description::Description;
use crate::cards::properties::duration::Duration;
use crate::cards::properties::effect::Effect;
use crate::cards::properties::effect_description::EffectDescription;
use crate::cards::properties::target::Target;
use crate::cards::properties::title::Title;
use crate::cards::types::card_model::Card;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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

#[cfg(test)]
pub(crate) mod tests {
    use crate::cards::properties::description::tests::FakeDescription;
    use crate::cards::properties::duration::tests::FakeDuration;
    use crate::cards::properties::effect::tests::FakeEffect;
    use crate::cards::properties::title::tests::FakeTitle;

    use super::*;

    pub struct FakeAttackCard;

    impl Dummy<FakeAttackCard> for AttackCard {
        fn dummy_with_rng<R: Rng + ?Sized>(_: &FakeAttackCard, _: &mut R) -> Self {
            AttackCard {
                title: FakeTitle.fake(),
                description: FakeDescription.fake(),
                effect: FakeEffect.fake(),
                duration: FakeDuration.fake(),
            }
        }
    }
}
