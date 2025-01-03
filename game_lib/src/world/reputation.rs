use std::fmt::Display;
use std::ops::{Add, Sub};

const MAX_VALUE: u8 = 100;
const MIN_VALUE: u8 = 0;

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct Reputation(u8);

impl Reputation {
    pub fn new(rep: u8) -> Self {
        if rep > MAX_VALUE {
            Reputation(MAX_VALUE)
        } else {
            Reputation(rep)
        }
    }

    pub fn start_value() -> Self {
        Reputation(50)
    }
    pub fn value(&self) -> &u8 {
        &self.0
    }
}

impl Add for &Reputation {
    type Output = Reputation;

    fn add(self, rhs: Self) -> Self::Output {
        let sum = self.0 + rhs.0;
        let new_value = if sum > MAX_VALUE { MAX_VALUE } else { sum };
        Reputation::new(new_value)
    }
}

impl Sub for &Reputation {
    type Output = Reputation;

    fn sub(self, rhs: Self) -> Self::Output {
        let sum = self.0 as i16 - rhs.0 as i16;
        let new_value = if sum < MIN_VALUE as i16 { MIN_VALUE } else { sum as u8 };
        Reputation::new(new_value)
    }
}

impl Display for Reputation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} reputation", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn create_reputation_max_value() {
        let rep = Reputation::new(100);
        assert_eq!(rep.value(), &100);
    }

    #[test]
    pub fn create_reputation_max_value_plus_1_returns_max_value() {
        let rep = Reputation::new(101);
        assert_eq!(rep.value(), &100);
    }

    #[test]
    pub fn add_10_to_20_should_return_30() {
        let input = Reputation::new(20);
        let add_sum = Reputation::new(10);
        let expected = Reputation::new(30);

        let result = &input + &add_sum;

        assert_eq!(result, expected);
    }

    #[test]
    pub fn add_10_to_99_should_return_max_value() {
        let input = Reputation::new(99);
        let add_sum = Reputation::new(10);
        let expected = Reputation::new(MAX_VALUE);

        let result = &input + &add_sum;

        assert_eq!(result, expected);
    }

    #[test]
    pub fn sub_10_from_30_should_return_20() {
        let input = Reputation::new(30);
        let sub = Reputation::new(10);
        let expected = Reputation::new(20);

        let result = &input - &sub;

        assert_eq!(result, expected);
    }

    #[test]
    pub fn sub_20_from_10_should_return_min_value() {
        let input = Reputation::new(10);
        let sub = Reputation::new(20);
        let expected = Reputation::new(MIN_VALUE);

        let result = &input - &sub;

        assert_eq!(result, expected);
    }

    #[test]
    pub fn display_reputation() {
        let rep = Reputation::new(10);
        assert_eq!(format!("{}", rep), "10 reputation");
    }
}
