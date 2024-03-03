use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub enum Duration {
    Rounds(usize),
    UntilClosed,
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
            Some(r) => Duration::Rounds(r),
        }
    }

    pub fn for_game() -> Self {
        Duration::UntilClosed
    }

    pub fn value(&self) -> Option<&usize> {
        match &self {
            Duration::Rounds(r) => Some(r),
            Duration::UntilClosed => None,
        }
    }
}
