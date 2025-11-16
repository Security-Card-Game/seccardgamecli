use serde::{Deserialize, Serialize};

use crate::cards::properties::description::Description;
use crate::cards::properties::effect::Effect;
use crate::cards::properties::title::Title;
use crate::cards::types::card_model::Card;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct EventCard {
    pub title: Title,
    pub description: Description,
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

    pub fn is_closeable(&self) -> bool {
        match &self.effect {
            | Effect::OnUsingForFix(_, _)
            | Effect::Other(_)
            | Effect::NOP
            | Effect::Immediate(_)
            | Effect::AttackSurface(_, _)
            | Effect::Incident(_, _, _) => true,
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

    pub struct FakeEventCard;
    pub struct FakeNoOpEventCard;

    impl Dummy<FakeEventCard> for EventCard {
        fn dummy_with_rng<R: Rng + ?Sized>(_: &FakeEventCard, _: &mut R) -> Self {
            EventCard {
                title: FakeTitle.fake(),
                description: FakeDescription.fake(),
                effect: FakeEffect.fake(),
            }
        }
    }

    impl Dummy<FakeNoOpEventCard> for EventCard {
        fn dummy_with_rng<R: Rng + ?Sized>(_: &FakeNoOpEventCard, _: &mut R) -> Self {
            EventCard {
                title: FakeTitle.fake(),
                description: FakeDescription.fake(),
                effect: Effect::NOP
            }
        }
    }

}
