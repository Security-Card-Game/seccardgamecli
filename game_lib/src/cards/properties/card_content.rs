use serde::{Deserialize, Serialize};

use crate::cards::errors::{ErrorKind, ModelError};
use crate::cards::world::game_model::Resources;

#[derive(Debug, Clone)]
pub struct Title(String);

#[derive(Debug, Clone)]
pub struct Description(String);

#[derive(Debug, Clone)]
pub struct ActionDescription(String);

#[derive(Debug, Clone)]
pub struct Target(String);

#[derive(Debug, Clone)]
pub struct FixModifier(isize);

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Duration {
    Rounds(usize),
    UntilClosed,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Action {
    Immediate(ActionDescription),
    OnTargetAvailable(ActionDescription),
    OnNextFix(ActionDescription, FixModifier),
    OnUsingForFix(ActionDescription, FixModifier),
    Other(ActionDescription),
    NOP,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FixCost {
    pub min: Resources,
    pub max: Resources,
}

impl Default for Duration {
    fn default() -> Self {
        Duration::UntilClosed
    }
}

impl Default for Action {
    fn default() -> Self {
        Action::NOP
    }
}

impl Title {
    pub fn new(title: &str) -> Self {
        Self(title.to_string())
    }

    pub(crate) fn empty() -> Self {
        Self("".to_string())
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl Description {
    pub fn new(title: &str) -> Self {
        Self(title.to_string())
    }

    pub fn empty() -> Self {
        Self("".to_string())
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl ActionDescription {
    pub fn new(title: &str) -> Self {
        Self(title.to_string())
    }

    pub fn empty() -> Self {
        Self("".to_string())
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl Target {
    pub fn new(title: &str) -> Self {
        Self(title.to_string())
    }

    pub fn empty() -> Self {
        Self("".to_string())
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl FixModifier {
    pub fn increase(inc: usize) -> Self {
        Self(inc as isize)
    }

    pub fn decrease(dec: usize) -> Self {
        Self(-1 * dec as isize)
    }

    pub fn new(value: isize) -> Self {
        FixModifier(value)
    }

    pub fn empty() -> Self {
        Self(0)
    }

    pub fn value(&self) -> &isize {
        &self.0
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

impl FixCost {
    pub fn new(min: usize, max: usize) -> Result<Self, ModelError> {
        if min > max {
            Err(ModelError {
                kind: ErrorKind::Validation,
                message: format!("min {} grater then max {}", min, max),
            })
        } else {
            Ok(FixCost {
                min: Resources::new(min),
                max: Resources::new(max),
            })
        }
    }

    pub fn min_value(&self) -> &usize {
        &self.min.value()
    }

    pub fn max_value(&self) -> &usize {
        &self.max.value()
    }
}

impl Default for FixCost {
    fn default() -> Self {
        FixCost {
            min: Resources::default(),
            max: Resources::default(),
        }
    }
}

impl Resources {
    pub fn value(&self) -> &usize {
        &self.0
    }
}

impl From<String> for Title {
    fn from(value: String) -> Self {
        Title(value)
    }
}

impl From<String> for Description {
    fn from(value: String) -> Self {
        Description(value)
    }
}

impl From<String> for ActionDescription {
    fn from(value: String) -> Self {
        ActionDescription(value)
    }
}

impl From<String> for Target {
    fn from(value: String) -> Self {
        Target(value)
    }
}
