use serde::{Deserialize, Serialize};
use crate::cards::properties::description::Description;
use crate::cards::properties::title::Title;
use crate::world::reputation::Reputation;
use crate::world::resource_fix_multiplier::ResourceFixMultiplier;
use crate::world::resources::Resources;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Scenario {
    pub title: Title,
    pub description: Description,
    pub preset: Preset,
    pub goal: Goal,
}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Preset {
    pub resources: Resources,
    pub reputation: Reputation,
    pub resource_gain: Resources,
    pub multiplier: ResourceFixMultiplier
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Goal {
    pub minimum_resources: Resources,
    pub minimum_reputation: Reputation
}