use serde::{Deserialize, Deserializer, Serialize, Serializer};
use crate::cards::serialization::helper::StrVisitor;

#[derive(Debug, Clone)]
pub struct Target(String);

impl Target {
    pub fn new(title: &str) -> Self {
        Self(title.to_string())
    }

    pub fn empty() -> Self {
        Self("".to_string())
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl From<String> for Target {
    fn from(value: String) -> Self {
        Target(value)
    }
}

impl Serialize for Target {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        serializer.serialize_str(&self.value())
    }
}
impl<'de> Deserialize<'de> for Target {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        deserializer.deserialize_string(StrVisitor(std::marker::PhantomData))
    }
}

