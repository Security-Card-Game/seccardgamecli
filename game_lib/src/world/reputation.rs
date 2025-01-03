use std::fmt::Display;
use std::ops::{Add, Sub};

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
struct Reputation(usize);


impl Reputation {
    pub fn new(rep: usize) -> Self { Reputation(rep) }
    pub fn value(&self) -> &usize { &self.0 }
}

impl Add for &Reputation {
    type Output = Reputation;

    fn add(self, rhs: Self) -> Self::Output {
        let sum = self.0 + rhs.0;
        Reputation(sum)
    }
}

impl Sub for &Reputation {
    type Output = Reputation;

    fn sub(self, rhs: Self) -> Self::Output {
        let sum = self.0 - rhs.0;
        Reputation(sum)
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
    pub fn create_reputation() {
        let rep = Reputation::new(10);
        assert_eq!(rep.value(), &10);
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
