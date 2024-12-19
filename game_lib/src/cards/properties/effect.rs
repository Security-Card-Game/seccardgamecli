use serde::{Deserialize, Serialize};
use crate::cards::properties::attack_costs::IncidentImpact;
use crate::cards::properties::effect_description::EffectDescription;
use crate::cards::properties::fix_modifier::FixModifier;
use crate::cards::properties::target::Target;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[derive(Default)]
pub enum Effect {
    Immediate(EffectDescription),
    AttackSurface(EffectDescription, Vec<Target>),
    Incident(EffectDescription, Vec<Target>, IncidentImpact),
    OnNextFix(EffectDescription, FixModifier),
    OnUsingForFix(EffectDescription, FixModifier),
    Other(EffectDescription),
    #[default]
    NOP,
}


#[cfg(test)]
pub(crate) mod tests {
    use fake::{Dummy, Fake};
    use rand::Rng;
    use crate::cards::properties::effect::Effect::{AttackSurface, Immediate, Incident, NOP, OnNextFix, OnUsingForFix, Other};
    use crate::cards::properties::effect_description::tests::FakeEffectDescription;
    use crate::cards::properties::fix_modifier::tests::FakeFixModifier;
    use crate::cards::properties::target::tests::FakeTarget;

    use super::*;

    pub struct FakeEffect;

    impl Dummy<FakeEffect> for Effect {
        fn dummy_with_rng<R: Rng + ?Sized>(_: &FakeEffect, rng: &mut R) -> Self {
            let description = FakeEffectDescription.fake();
            let target = FakeTarget.fake();
            let modifier = FakeFixModifier.fake();
            // this might not be optimal but it is quick
            match rng.gen_range(0..7) {
                0 => Immediate(description),
                1 => AttackSurface(description, vec![target]),
                2 => Incident(description, vec![target]),
                3 => OnNextFix(description, modifier),
                4 => OnUsingForFix(description, modifier),
                5 => Other(description),
                6 => NOP,
                _ => panic!("Only seven enums are known at the moment"),
            }
        }
    }
}
