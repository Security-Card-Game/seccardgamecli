use std::ops::{Add, Mul};

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
            FixModifier::Increase(r) => *r.value() as isize,
            FixModifier::Decrease(r) => -(*r.value() as isize),
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

impl Add for FixModifier {
    type Output = FixModifier;

    fn add(self, rhs: Self) -> Self::Output {
        let new_value = self.value() + rhs.value();

        if new_value <= 0 {
            FixModifier::Decrease(Resources::new(new_value.unsigned_abs()))
        } else {
            FixModifier::Increase(Resources::new(new_value.unsigned_abs()))
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
            };
        }
    }

    #[test]
    fn add_two_increasing() {
        let add1 = FixModifier::Increase(Resources::new(10));
        let add2 = FixModifier::Increase(Resources::new(1));

        let sum = add1 + add2;

        assert_eq!(sum, FixModifier::Increase(Resources::new(11)))
    }

    #[test]
    fn add_two_decreasing() {
        let add1 = FixModifier::Decrease(Resources::new(9));
        let add2 = FixModifier::Decrease(Resources::new(4));

        let sum = add1 + add2;

        assert_eq!(sum, FixModifier::Decrease(Resources::new(13)))
    }

    #[test]
    fn add_increasing_to_higher_decreasing() {
        let add1 = FixModifier::Decrease(Resources::new(9));
        let add2 = FixModifier::Increase(Resources::new(4));

        let sum = add1 + add2;

        assert_eq!(sum, FixModifier::Decrease(Resources::new(5)))
    }

    #[test]
    fn add_increasing_to_lower_decreasing() {
        let add1 = FixModifier::Decrease(Resources::new(2));
        let add2 = FixModifier::Increase(Resources::new(4));

        let sum = add1 + add2;

        assert_eq!(sum, FixModifier::Increase(Resources::new(2)))
    }

    #[test]
    fn add_decreasing_to_higher_increasing() {
        let add1 = FixModifier::Increase(Resources::new(9));
        let add2 = FixModifier::Decrease(Resources::new(4));

        let sum = add1 + add2;

        assert_eq!(sum, FixModifier::Increase(Resources::new(5)))
    }

    #[test]
    fn add_decreasing_to_same_value_increasing() {
        let add1 = FixModifier::Increase(Resources::new(9));
        let add2 = FixModifier::Decrease(Resources::new(9));

        let sum = add1 + add2;

        assert_eq!(sum, FixModifier::Decrease(Resources::new(0)))
    }

    #[test]
    fn add_increasing_to_same_value_decreasing() {
        let add1 = FixModifier::Decrease(Resources::new(9));
        let add2 = FixModifier::Increase(Resources::new(9));

        let sum = add1 + add2;

        assert_eq!(sum, FixModifier::Decrease(Resources::new(0)))
    }
}
