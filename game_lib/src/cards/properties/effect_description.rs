use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::cards::serialization::helper::StrVisitor;

#[derive(Debug, Clone, PartialEq)]
pub struct EffectDescription(String);

impl EffectDescription {
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

impl From<String> for EffectDescription {
    fn from(value: String) -> Self {
        EffectDescription(value)
    }
}

impl Serialize for EffectDescription {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.value())
    }
}
impl<'de> Deserialize<'de> for EffectDescription {
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
    use rand::Rng;

    use super::*;

    pub struct FakeEffectDescription;
    impl Dummy<FakeEffectDescription> for EffectDescription {
        fn dummy_with_rng<R: Rng + ?Sized>(_: &FakeEffectDescription, _: &mut R) -> Self {
            let words: Vec<String> = Words(3..10).fake();
            EffectDescription(words.join(" ").to_string())
        }
    }
}
