use std::ops::Mul;

use serde::{Deserialize, Serialize};

use crate::cards::errors::{ErrorKind, ModelError};
use crate::world::resource_fix_multiplier::ResourceFixMultiplier;
use crate::world::resources::Resources;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
#[derive(Default)]
pub struct FixCost {
    pub min: Resources,
    pub max: Resources,
}

impl FixCost {
    pub fn new(min: usize, max: usize) -> Result<Self, ModelError> {
        if min > max {
            Err(ModelError {
                kind: ErrorKind::Validation,
                message: format!("min {} grater then or equal max {}", min, max),
            })
        } else {
            Ok(FixCost {
                min: Resources::new(min),
                max: Resources::new(max),
            })
        }
    }

    pub fn from_resources(min: Resources, max: Resources) -> Result<Self, ModelError> {
        if min > max {
            Err(ModelError {
                kind: ErrorKind::Validation,
                message: format!("min {:?} grater then max {:?}", min, max),
            })
        } else {
            Ok(FixCost { min, max })
        }
    }

    pub fn min_value(&self) -> &usize {
        self.min.value()
    }

    pub fn max_value(&self) -> &usize {
        self.max.value()
    }
}


impl Mul<&ResourceFixMultiplier> for FixCost {
    type Output = Self;

    fn mul(self, rhs: &ResourceFixMultiplier) -> Self::Output {
        FixCost {
            min: self.min * rhs,
            max: self.max * rhs,
        }
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use fake::Dummy;
    use rand::Rng;

    use super::*;

    pub struct FakeFixCost;

    impl Dummy<FakeFixCost> for FixCost {
        fn dummy_with_rng<R: Rng + ?Sized>(_: &FakeFixCost, rng: &mut R) -> Self {
            FixCost::from_resources(
                Resources::new(rng.gen_range(1..10)),
                Resources::new(rng.gen_range(11..20)),
            )
            .unwrap()
        }
    }
}
