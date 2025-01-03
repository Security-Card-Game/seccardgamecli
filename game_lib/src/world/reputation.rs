use std::fmt::Display;
use std::ops::{Add, Sub};

const MAX_VALUE: u8 = 100;

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct Reputation(u8);

impl Reputation {
    pub fn new(rep: u8) -> Self {
        if rep > MAX_VALUE {
            panic!("Reputation can't be higher than {}", MAX_VALUE);
        }
        Reputation(rep)
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
        if sum > MAX_VALUE {
            Reputation::new(100)
        } else {
            Reputation::new(sum)
        }
    }
}

impl Sub for &Reputation {
    type Output = Reputation;

    fn sub(self, rhs: Self) -> Self::Output {
        let sum = self.0 - rhs.0;
        Reputation::new(sum)
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
    #[should_panic]
    pub fn cannot_create_reputation_above_100() {
        Reputation::new(101);
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
    pub fn add_10_to_99_should_return_100() {
        let input = Reputation::new(99);
        let add_sum = Reputation::new(10);
        let expected = Reputation::new(100);

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
    #[should_panic]
    pub fn sub_20_from_10_should_panic() {
        let input = Reputation::new(10);
        let sub = Reputation::new(20);

        &input - &sub;
    }

    #[test]
    pub fn display_reputation() {
        let rep = Reputation::new(10);
        assert_eq!(format!("{}", rep), "10 reputation");
    }
}
