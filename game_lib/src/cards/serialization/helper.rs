use std::error::Error;
use std::fmt;
use std::marker::PhantomData;
use serde::de::Visitor;

pub(crate) struct StrVisitor<T>(pub(crate) PhantomData<T>);

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

pub(crate) struct NumberVisitor<T: Number>(pub(crate) PhantomData<T>);

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


