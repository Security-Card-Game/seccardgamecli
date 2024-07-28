use rand::Rng;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::cards::serialization::helper::StrVisitor;

#[derive(Debug, Clone, PartialEq)]
pub struct Description(String);

impl Description {
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

impl From<String> for Description {
    fn from(value: String) -> Self {
        Description(value)
    }
}

impl Serialize for Description {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.value())
    }
}
impl<'de> Deserialize<'de> for Description {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_string(StrVisitor(std::marker::PhantomData))
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use fake::Dummy;
    use fake::Fake;
    use fake::faker::lorem::en::*;

    use super::*;

    pub struct FakeDescription;
    impl Dummy<FakeDescription> for Description {
        fn dummy_with_rng<R: Rng + ?Sized>(_: &FakeDescription, _: &mut R) -> Self {
            let words: Vec<String> = Words(10..20).fake();
            Description(words.join(" ").to_string())
        }
    }
}
