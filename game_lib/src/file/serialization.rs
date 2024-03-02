use std::error::Error;
use std::fmt;
use std::marker::PhantomData;

use serde::{de::Visitor, Deserialize, Deserializer, Serialize, Serializer};
use crate::cards::card_content::{Action, ActionDescription, Description, FixModifier, Target, Title};
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

pub trait Number: Sized {
    fn from_i64(value: i64) -> Self;
    fn from_u64(value: u64) -> Self;
}

impl Number for i64 {
    fn from_i64(value: i64) -> Self {
        value
    }

    fn from_u64(value: u64) -> Self {
        value as Self
    }
}

impl Number for u64 {
    fn from_i64(value: i64) -> Self {
        value as Self
    }

    fn from_u64(value: u64) -> Self {
        value
    }
}

struct NumberVisitor<T: Number>(std::marker::PhantomData<T>);

impl<'de, T> Visitor<'de> for NumberVisitor<T>
    where T: Number {
    type Value = T;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("any JSON number")
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
        where
            E: Error,
    {
        Ok(T::from_i64(value))
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
        where
            E: Error,
    {
        Ok(T::from_u64(value))
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
        deserializer.deserialize_string(StrVisitor(PhantomData))
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
        deserializer.deserialize_u64(NumberVisitor(std::marker::PhantomData))
    }
}

impl Number for Resources {
    fn from_i64(value: i64) -> Self {
        panic!("Must be positive value")
    }

    fn from_u64(value: u64) -> Self {
        Resources::new(value as usize)
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
