use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub enum Duration {
    Rounds(usize),
    UntilClosed,
    None,
}

impl Default for Duration {
    fn default() -> Self {
        Duration::UntilClosed
    }
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
