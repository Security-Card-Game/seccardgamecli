use std::fmt::{Display, Formatter};
use std::ops::{Add, Mul, Sub};

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::cards::serialization::helper::Number;
use crate::world::resource_fix_multiplier::ResourceFixMultiplier;

#[derive(Clone, Debug, PartialOrd, PartialEq, Default)]
pub struct Resources(usize);

impl Resources {
    pub fn new(value: usize) -> Self {
        Resources(value)
    }
    pub fn value(&self) -> &usize {
        &self.0
    }
}

impl Add for Resources {
    type Output = Resources;

    fn add(self, rhs: Self) -> Self::Output {
        Resources(self.0 + rhs.0)
    }
}

impl Add for &Resources {
    type Output = Resources;

    fn add(self, rhs: Self) -> Self::Output {
        Resources(self.0 + rhs.0)
    }
}

impl Sub for Resources {
    type Output = Resources;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.0 <= rhs.0 {
            Resources::new(0)
        } else {
            Resources(self.0 - rhs.0)
        }
    }
}

impl Sub for &Resources {
    type Output = Resources;
    fn sub(self, rhs: Self) -> Self::Output {
        if self.0 <= rhs.0 {
            Resources::new(0)
        } else {
            Resources(self.0 - rhs.0)
        }
    }
}

impl Serialize for Resources {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u64(*self.value() as u64)
    }
}
impl<'de> Deserialize<'de> for Resources {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_u64(crate::cards::serialization::helper::NumberVisitor(
            std::marker::PhantomData,
        ))
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

impl Mul<ResourceFixMultiplier> for Resources {
    type Output = Self;

    fn mul(self, rhs: ResourceFixMultiplier) -> Self::Output {
        Resources(self.0 * rhs.value())
    }
}


impl Mul<usize> for &Resources {
    type Output = Resources;

    fn mul(self, rhs: usize) -> Self::Output {
        Resources(self.0 * rhs)
    }
}

impl Mul<&ResourceFixMultiplier> for Resources {
    type Output = Self;

    fn mul(self, rhs: &ResourceFixMultiplier) -> Self::Output {
        Resources(self.0 * rhs.value())
    }
}

impl Mul<&ResourceFixMultiplier> for &Resources {
    type Output = Resources;

    fn mul(self, rhs: &ResourceFixMultiplier) -> Self::Output {
        Resources(self.0 * rhs.value())
    }
}

impl Display for Resources {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} resources", self.0)
    }
}
