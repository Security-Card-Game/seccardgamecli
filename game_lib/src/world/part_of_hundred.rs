use std::fmt::Display;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use crate::cards::serialization::helper::{Number, NumberVisitor};

#[derive(Clone, Debug, PartialEq)]
pub struct PartOfHundred {
    pub value: u8,
}

impl Display for PartOfHundred {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}%", self.value)
    }
}

impl PartOfHundred {
    pub fn new(value: u8) -> Self {
        if value > 100 {
            panic!("Value must be between 0 and 100");
        }
        PartOfHundred { value }
    }

    fn guard(value: i64) {
        if (value > 100) || (value < 0) {
            panic!("Value must be between 0 and 100");
        }
    }
}

impl Number for PartOfHundred {
    fn from_i64(value: i64) -> Self {
        Self::guard(value);
        PartOfHundred::new(value as u8)
    }

    fn from_u64(value: u64) -> Self {
        Self::guard(value as i64);
        PartOfHundred::new(value as u8)
    }
}

impl<'de> Deserialize<'de> for PartOfHundred {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        deserializer.deserialize_u64(NumberVisitor(std::marker::PhantomData))
    }
}

impl Serialize for PartOfHundred {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        serializer.serialize_u64(self.value as u64)
    }
}


#[cfg(test)]
mod tests {
    use std::panic;
    use quickcheck::{Arbitrary, Gen};
    use quickcheck_macros::quickcheck;
    use super::*;

    #[derive(Clone, Debug, PartialEq)]
    struct ValidValues {
        prop: u8,
    }
    #[derive(Clone, Debug, PartialEq)]
    struct InvalidValues {
        prop: u8
    }

    impl Arbitrary for ValidValues {
        fn arbitrary(g: &mut Gen) -> Self {
            let value = u8::arbitrary(g) % 101;
            ValidValues {
                prop: value
            }
        }
    }

    impl Arbitrary for InvalidValues {
        fn arbitrary(g: &mut Gen) -> Self {
            let value = u8::arbitrary(g) % 101 + 101;
            InvalidValues {
                prop: value
            }
        }
    }

    #[quickcheck]
    fn create_part_of_hundred_with_values_between_0_and_100_works(value: ValidValues) -> bool {
        let sut = PartOfHundred::new(value.prop);
        sut.value == value.prop
    }

    #[quickcheck]
    fn create_part_of_hundred_with_values_grater_than_100_fails(value: InvalidValues) -> bool {
        let result = panic::catch_unwind(|| {
            PartOfHundred::new(value.prop);
        });
        result.is_err()
    }
}