use crate::cards::properties::incident_impact::IncidentImpact;
use crate::cards::properties::effect_description::EffectDescription;
use crate::cards::properties::cost_modifier::CostModifier;
use crate::cards::properties::target::Target;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[derive(Default)]
pub enum Effect {
    Immediate(EffectDescription),
    AttackSurface(EffectDescription, Vec<Target>),
    Incident(EffectDescription, Vec<Target>, IncidentImpact),
    OnNextFix(EffectDescription, CostModifier),
    OnUsingForFix(EffectDescription, CostModifier),
    Other(EffectDescription),
    #[default]
    NOP,
}


#[cfg(test)]
pub(crate) mod tests {
    use crate::cards::properties::incident_impact::tests::FakeFixedIncidentImpact;
    use crate::cards::properties::effect::Effect::{AttackSurface, Immediate, Incident, OnNextFix, OnUsingForFix, Other, NOP};
    use crate::cards::properties::effect_description::tests::FakeEffectDescription;
    use crate::cards::properties::cost_modifier::tests::FakeCostModifier;
    use crate::cards::properties::target::tests::FakeTarget;
    use fake::{Dummy, Fake};
    use rand::Rng;

    use super::*;

    pub struct FakeEffect;

    impl Dummy<FakeEffect> for Effect {
        fn dummy_with_rng<R: Rng + ?Sized>(_: &FakeEffect, rng: &mut R) -> Self {
            let description = FakeEffectDescription.fake();
            let target = FakeTarget.fake();
            let modifier = FakeCostModifier.fake();
            // this might not be optimal but it is quick
            match rng.gen_range(0..7) {
                0 => Immediate(description),
                1 => AttackSurface(description, vec![target]),
                2 => Incident(description, vec![target], FakeFixedIncidentImpact.fake()),
                3 => OnNextFix(description, modifier),
                4 => OnUsingForFix(description, modifier),
                5 => Other(description),
                6 => NOP,
                _ => panic!("Only seven enums are known at the moment"),
            }
        }
    }
}
