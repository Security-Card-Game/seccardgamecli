use std::ops::Mul;

use serde::{Deserialize, Serialize};

use crate::world::resource_fix_multiplier::ResourceFixMultiplier;
use crate::world::resources::Resources;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub enum FixModifier {
    Increase(Resources),
    Decrease(Resources),
}

impl FixModifier {
    pub fn value(&self) -> isize {
        match self {
            FixModifier::Increase(r) => r.value().clone() as isize,
            FixModifier::Decrease(r) => -1 * r.value().clone() as isize,
        }
    }
}

impl Mul<ResourceFixMultiplier> for FixModifier {
    type Output = Self;

    fn mul(self, rhs: ResourceFixMultiplier) -> Self::Output {
        match self {
            FixModifier::Increase(r) => FixModifier::Increase(r * rhs),
            FixModifier::Decrease(r) => FixModifier::Decrease(r * rhs),
        }
    }
}
