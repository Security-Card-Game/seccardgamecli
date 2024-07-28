use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::cards::serialization::helper::StrVisitor;

#[derive(Debug, Clone, PartialEq)]
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

#[cfg(test)]
pub(crate) mod tests {
    use fake::Dummy;
    use rand::Rng;

    use super::*;

    const TEST_TARGETS: [&'static str; 5] = [
        "backend",
        "frontend",
        "infrastructure",
        "social",
        "supply chain",
    ];

    pub struct FakeTarget;
    impl Dummy<FakeTarget> for Target {
        fn dummy_with_rng<R: Rng + ?Sized>(_: &FakeTarget, rng: &mut R) -> Self {
            let target = TEST_TARGETS[rng.gen_range(0..TEST_TARGETS.len())];
            Target(target.to_string())
        }
    }
}
