use serde::{Deserialize, Deserializer, Serialize, Serializer};
use crate::cards::properties::attack_costs::AttackCost::Fixed;
use crate::cards::serialization::helper::{Number, NumberVisitor};
use crate::world::resources::Resources;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum AttackCost {
    PartOfRevenue(PartOfHundred),
    Fixed(Resources)
}

impl AttackCost {
    pub fn none() -> Self {
        Fixed(Resources::new(0))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PartOfHundred {
    value: u8,
}

impl PartOfHundred {
    pub fn new(value: u8) -> Self {
        if (value > 100) || (value < 0) {
            panic!("Value must be between 0 and 100");
        }
        PartOfHundred { value }
    }

    fn guard(value: i64) {
        if (value > 100) || (value < 0) {
            panic!("Value must be between 0 and 100");
        }
    }
}

impl Number for PartOfHundred {
    fn from_i64(value: i64) -> Self {
        Self::guard(value);
        PartOfHundred { value: value as u8 }
    }

    fn from_u64(value: u64) -> Self {
        Self::guard(value as i64);
        PartOfHundred { value: value as u8 }
    }
}

impl<'de> Deserialize<'de> for PartOfHundred {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        deserializer.deserialize_u64(NumberVisitor(std::marker::PhantomData))
    }
}

impl Serialize for PartOfHundred {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        serializer.serialize_u64(self.value as u64)
    }
}