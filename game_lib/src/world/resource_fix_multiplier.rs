use crate::cards::serialization::helper::Number;
use log::warn;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Clone, Debug, PartialOrd, PartialEq, Copy)]
pub struct ResourceFixMultiplier(usize);

impl ResourceFixMultiplier {
    pub fn new(value: usize) -> Self {
        if value < 1 {
            warn!("Modifier must not be 0. Setting it to 1!")
        }
        ResourceFixMultiplier(value)
    }

    pub fn value(&self) -> &usize {
        &self.0
    }
}

impl Serialize for ResourceFixMultiplier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u64(*self.value() as u64)
    }
}
impl<'de> Deserialize<'de> for ResourceFixMultiplier {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_u64(crate::cards::serialization::helper::NumberVisitor(
            std::marker::PhantomData,
        ))
    }
}

impl Number for ResourceFixMultiplier {
    fn from_i64(_value: i64) -> Self {
        panic!("Must be positive value")
    }

    fn from_u64(value: u64) -> Self {
        ResourceFixMultiplier::new(value as usize)
    }
}

impl Default for ResourceFixMultiplier {
    fn default() -> Self {
        ResourceFixMultiplier(1)
    }
}
