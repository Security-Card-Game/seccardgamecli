use std::ops::Mul;

use serde::{Deserialize, Serialize};

use crate::world::resource_fix_multiplier::ResourceFixMultiplier;
use crate::world::resources::Resources;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
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

impl Mul<&ResourceFixMultiplier> for FixModifier {
    type Output = Self;

    fn mul(self, rhs: &ResourceFixMultiplier) -> Self::Output {
        match self {
            FixModifier::Increase(r) => FixModifier::Increase(r * rhs),
            FixModifier::Decrease(r) => FixModifier::Decrease(r * rhs),
        }
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use fake::Dummy;
    use rand::Rng;

    use super::*;

    pub struct FakeFixModifier;

    impl Dummy<FakeFixModifier> for FixModifier {
        fn dummy_with_rng<R: Rng + ?Sized>(_: &FakeFixModifier, rng: &mut R) -> Self {
            return if rng.gen_bool(1.0 / 2.0) {
                FixModifier::Decrease(Resources::new(rng.gen_range(1..10)))
            } else {
                FixModifier::Increase(Resources::new(rng.gen_range(1..10)))
            }
        }
    }
}