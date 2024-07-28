use std::marker::PhantomData;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Clone, PartialEq)]
pub struct Title(String);

impl Title {
    pub fn new(title: &str) -> Self {
        Self(title.to_string())
    }

    pub(crate) fn empty() -> Self {
        Self("".to_string())
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl From<String> for Title {
    fn from(value: String) -> Self {
        Title(value)
    }
}

impl Serialize for Title {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.value())
    }
}

impl<'de> Deserialize<'de> for Title {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer
            .deserialize_string(crate::cards::serialization::helper::StrVisitor(PhantomData))
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use fake::Dummy;
    use fake::Fake;
    use fake::faker::lorem::en::*;
    use rand::Rng;

    use super::*;

    pub struct FakeTitle;
    impl Dummy<FakeTitle> for Title {
        fn dummy_with_rng<R: Rng + ?Sized>(_: &FakeTitle, _: &mut R) -> Self {
            let words: Vec<String> = Words(1..5).fake();
            Title(words.join(" "))
        }
    }
}
