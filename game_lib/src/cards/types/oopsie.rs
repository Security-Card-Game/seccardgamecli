use serde::{Deserialize, Serialize};

use crate::cards::properties::description::Description;
use crate::cards::properties::effect::Effect;
use crate::cards::properties::effect_description::EffectDescription;
use crate::cards::properties::fix_cost::FixCost;
use crate::cards::properties::target::Target;
use crate::cards::properties::title::Title;
use crate::cards::types::card_model::Card;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct OopsieCard {
    pub title: Title,
    pub description: Description,
    pub effect: Effect,
    pub fix_cost: FixCost,
}

impl OopsieCard {
    pub fn new(
        title: Title,
        description: Description,
        targets: Vec<Target>,
        effect: EffectDescription,
        fix_cost: FixCost,
    ) -> Self {
        OopsieCard {
            title,
            description,
            effect: Effect::AttackSurface(effect, targets),
            fix_cost,
        }
    }

    pub fn empty() -> Card {
        Card::Oopsie(OopsieCard {
            title: Title::empty(),
            description: Description::empty(),
            effect: Effect::default(),
            fix_cost: FixCost::default(),
        })
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use fake::{Dummy, Fake};
    use rand::Rng;
    use crate::cards::properties::description::tests::FakeDescription;
    use crate::cards::properties::effect::tests::FakeEffect;
    use crate::cards::properties::fix_cost::tests::FakeFixCost;
    use crate::cards::properties::title::tests::FakeTitle;

    use super::*;

    pub struct FakeOopsieCard;

    impl Dummy<FakeOopsieCard> for OopsieCard {
        fn dummy_with_rng<R: Rng + ?Sized>(_: &FakeOopsieCard, _: &mut R) -> Self {
            OopsieCard {
                title: FakeTitle.fake(),
                description: FakeDescription.fake(),
                effect: FakeEffect.fake(),
                fix_cost: FakeFixCost.fake(),
            }
        }
    }
}
