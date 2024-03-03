use std::marker::PhantomData;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use crate::cards::serialization::helper::{Number, NumberVisitor};

#[derive(Debug, Clone)]
pub struct FixModifier(isize);


impl FixModifier {
    pub fn increase(inc: usize) -> Self {
        Self(inc as isize)
    }

    pub fn decrease(dec: usize) -> Self {
        Self(-1 * dec as isize)
    }

    pub fn new(value: isize) -> Self {
        FixModifier(value)
    }

    pub fn empty() -> Self {
        Self(0)
    }

    pub fn value(&self) -> &isize {
        &self.0
    }
}

impl Serialize for FixModifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        serializer.serialize_i64(**&self.value() as i64)
    }
}

impl<'de> Deserialize<'de> for FixModifier {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        deserializer.deserialize_i64(NumberVisitor(PhantomData))
    }
}

impl Number for FixModifier {
    fn from_i64(value: i64) -> Self {
        FixModifier::new(value as isize)
    }

    fn from_u64(value: u64) -> Self {
        FixModifier::new(value as isize)
    }
}

impl From<String> for FixModifier {
    fn from(value: String) -> Self {
        match value.parse::<isize>() {
            Ok(s) => FixModifier::new(s),
            Err(_) => FixModifier::empty(),
        }
    }
}
