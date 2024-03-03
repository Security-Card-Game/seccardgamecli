use serde::{Deserialize, Serialize};
use crate::cards::properties::effect_description::EffectDescription;
use crate::cards::properties::fix_modifier::FixModifier;
use crate::cards::properties::target::Target;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum Effect {
    Immediate(EffectDescription),
    Incident(EffectDescription, Vec<Target>),
    OnNextFix(EffectDescription, FixModifier),
    OnUsingForFix(EffectDescription, FixModifier),
    Other(EffectDescription),
    NOP,
}

impl Default for Effect {
    fn default() -> Self {
        Effect::NOP
    }
}
