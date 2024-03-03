use serde::{Deserialize, Deserializer, Serialize, Serializer};
use crate::cards::serialization::helper::Number;

#[derive(Clone, Debug)]
pub struct Resources(usize);

impl Resources {
    pub fn new(value: usize) -> Self {
        Resources(value)
    }
    pub fn value(&self) -> &usize {
        &self.0
    }
}

impl Default for Resources {
    fn default() -> Self {
        Resources(0)
    }
}

impl Serialize for Resources {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        serializer.serialize_u64(**&self.value() as u64)
    }
}
impl<'de> Deserialize<'de> for Resources {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        deserializer.deserialize_u64(crate::cards::serialization::helper::NumberVisitor(std::marker::PhantomData))
    }
}

impl Number for Resources {
    fn from_i64(_value: i64) -> Self {
        panic!("Must be positive value")
    }

    fn from_u64(value: u64) -> Self {
        Resources::new(value as usize)
    }
}
