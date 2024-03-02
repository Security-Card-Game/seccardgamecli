use std::fmt;

use serde::{de::Visitor, Deserialize, Deserializer, Serialize, Serializer};

use crate::cards::card_content::{ActionDescription, Description, FixModifier, Target, Title};
use crate::cards::game_model::Resources;

struct StrVisitor<T>(std::marker::PhantomData<T>);

impl<'de, T> Visitor<'de> for StrVisitor<T>
where
    T: From<String>,
{
    type Value = T;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string")
    }

    fn visit_str<E: serde::de::Error>(self, value: &str) -> Result<T, E> {
        Ok(T::from(value.to_string()))
    }
}

impl Serialize for Title {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.value())
    }
}

impl<'de> Deserialize<'de> for Title {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_string(StrVisitor(std::marker::PhantomData))
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

impl Serialize for ActionDescription {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.value())
    }
}
impl<'de> Deserialize<'de> for ActionDescription {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_string(StrVisitor(std::marker::PhantomData))
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
        deserializer.deserialize_string(StrVisitor(std::marker::PhantomData))
    }
}

impl From<String> for Resources {
    fn from(value: String) -> Self {
        match value.parse::<usize>() {
            Ok(s) => Resources::new(s),
            Err(_) => Resources::default(),
        }
    }
}

impl Serialize for FixModifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u64(**&self.value() as u64)
    }
}

impl<'de> Deserialize<'de> for FixModifier {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_string(StrVisitor(std::marker::PhantomData))
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
