use serde::{Deserialize, Serialize};
use crate::cards::properties::effect_description::EffectDescription;
use crate::cards::properties::fix_modifier::FixModifier;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Effect {
    Immediate(EffectDescription),
    OnTargetAvailable(EffectDescription),
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
