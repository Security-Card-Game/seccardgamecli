use std::ops::{Add, Mul};

use serde::{Deserialize, Serialize};

use crate::world::resource_fix_multiplier::ResourceFixMultiplier;
use crate::world::resources::Resources;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum CostModifier {
    Increase(Resources),
    Decrease(Resources),
}

impl CostModifier {
    pub fn value(&self) -> isize {
        match self {
            CostModifier::Increase(r) => *r.value() as isize,
            CostModifier::Decrease(r) => -(*r.value() as isize),
        }
    }
}

impl Mul<ResourceFixMultiplier> for CostModifier {
    type Output = Self;

    fn mul(self, rhs: ResourceFixMultiplier) -> Self::Output {
        match self {
            CostModifier::Increase(r) => CostModifier::Increase(r * rhs),
            CostModifier::Decrease(r) => CostModifier::Decrease(r * rhs),
        }
    }
}

impl Mul<&ResourceFixMultiplier> for CostModifier {
    type Output = Self;

    fn mul(self, rhs: &ResourceFixMultiplier) -> Self::Output {
        match self {
            CostModifier::Increase(r) => CostModifier::Increase(r * rhs),
            CostModifier::Decrease(r) => CostModifier::Decrease(r * rhs),
        }
    }
}

impl Add for CostModifier {
    type Output = CostModifier;

    fn add(self, rhs: Self) -> Self::Output {
        let new_value = self.value() + rhs.value();

        if new_value <= 0 {
            CostModifier::Decrease(Resources::new(new_value.unsigned_abs()))
        } else {
            CostModifier::Increase(Resources::new(new_value.unsigned_abs()))
        }
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use fake::Dummy;
    use rand::Rng;

    use super::*;

    pub struct FakeCostModifier;

    impl Dummy<FakeCostModifier> for CostModifier {
        fn dummy_with_rng<R: Rng + ?Sized>(_: &FakeCostModifier, rng: &mut R) -> Self {
            return if rng.gen_bool(1.0 / 2.0) {
                CostModifier::Decrease(Resources::new(rng.gen_range(1..10)))
            } else {
                CostModifier::Increase(Resources::new(rng.gen_range(1..10)))
            };
        }
    }

    #[test]
    fn add_two_increasing() {
        let add1 = CostModifier::Increase(Resources::new(10));
        let add2 = CostModifier::Increase(Resources::new(1));

        let sum = add1 + add2;

        assert_eq!(sum, CostModifier::Increase(Resources::new(11)))
    }

    #[test]
    fn add_two_decreasing() {
        let add1 = CostModifier::Decrease(Resources::new(9));
        let add2 = CostModifier::Decrease(Resources::new(4));

        let sum = add1 + add2;

        assert_eq!(sum, CostModifier::Decrease(Resources::new(13)))
    }

    #[test]
    fn add_increasing_to_higher_decreasing() {
        let add1 = CostModifier::Decrease(Resources::new(9));
        let add2 = CostModifier::Increase(Resources::new(4));

        let sum = add1 + add2;

        assert_eq!(sum, CostModifier::Decrease(Resources::new(5)))
    }

    #[test]
    fn add_increasing_to_lower_decreasing() {
        let add1 = CostModifier::Decrease(Resources::new(2));
        let add2 = CostModifier::Increase(Resources::new(4));

        let sum = add1 + add2;

        assert_eq!(sum, CostModifier::Increase(Resources::new(2)))
    }

    #[test]
    fn add_decreasing_to_higher_increasing() {
        let add1 = CostModifier::Increase(Resources::new(9));
        let add2 = CostModifier::Decrease(Resources::new(4));

        let sum = add1 + add2;

        assert_eq!(sum, CostModifier::Increase(Resources::new(5)))
    }

    #[test]
    fn add_decreasing_to_same_value_increasing() {
        let add1 = CostModifier::Increase(Resources::new(9));
        let add2 = CostModifier::Decrease(Resources::new(9));

        let sum = add1 + add2;

        assert_eq!(sum, CostModifier::Decrease(Resources::new(0)))
    }

    #[test]
    fn add_increasing_to_same_value_decreasing() {
        let add1 = CostModifier::Decrease(Resources::new(9));
        let add2 = CostModifier::Increase(Resources::new(9));

        let sum = add1 + add2;

        assert_eq!(sum, CostModifier::Decrease(Resources::new(0)))
    }
}
