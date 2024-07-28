use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
#[derive(Default)]
pub enum Duration {
    Rounds(usize),
    #[default]
    UntilClosed,
    None,
}


impl Duration {
    pub fn new(round: Option<usize>) -> Self {
        match round {
            None => Duration::UntilClosed,
            Some(r) => {
                if r == 0 {
                    Duration::None
                } else {
                    Duration::Rounds(r)
                }
            }
        }
    }

    pub fn for_game() -> Self {
        Duration::UntilClosed
    }

    pub fn value(&self) -> Option<&usize> {
        match &self {
            Duration::Rounds(r) => Some(r),
            Duration::UntilClosed => None,
            Duration::None => None,
        }
    }

    pub fn decrease(&self) -> Self {
        match &self {
            Duration::Rounds(v) => Duration::new(Some(v - 1)),
            Duration::UntilClosed => Duration::UntilClosed,
            Duration::None => Duration::None,
        }
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use fake::Dummy;
    use rand::Rng;

    use super::*;

    pub struct FakeDuration;

    impl Dummy<FakeDuration> for Duration {
        fn dummy_with_rng<R: Rng + ?Sized>(_: &FakeDuration, rng: &mut R) -> Self {
            let type_id = rng.gen_range(0..3);
            return match type_id {
                0 => Duration::None,
                1 => Duration::UntilClosed,
                2 => Duration::Rounds(rng.gen_range(1..100)),
                _ => panic!("only three types 0 - 2 supported!"),
            };
        }
    }
}
