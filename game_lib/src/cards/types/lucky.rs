use serde::{Deserialize, Serialize};

use crate::cards::properties::description::Description;
use crate::cards::properties::effect::Effect;
use crate::cards::properties::title::Title;
use crate::cards::types::card_model::Card;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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

    pub fn is_closeable(&self) -> bool {
        match &self.effect {
            | Effect::OnUsingForFix(_, _)
            | Effect::Other(_)
            | Effect::NOP
            | Effect::Immediate(_)
            | Effect::AttackSurface(_, _)
            | Effect::Incident(_, _) => true,
            | Effect::OnNextFix(_, _) => false
        }
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use fake::{Dummy, Fake};
    use rand::Rng;

    use crate::cards::properties::description::tests::FakeDescription;
    use crate::cards::properties::effect::tests::FakeEffect;
    use crate::cards::properties::title::tests::FakeTitle;

    use super::*;

    pub struct FakeLuckyCard;

    impl Dummy<FakeLuckyCard> for LuckyCard {
        fn dummy_with_rng<R: Rng + ?Sized>(_: &FakeLuckyCard, _: &mut R) -> Self {
            LuckyCard {
                title: FakeTitle.fake(),
                description: FakeDescription.fake(),
                effect: FakeEffect.fake(),
            }
        }
    }
}
