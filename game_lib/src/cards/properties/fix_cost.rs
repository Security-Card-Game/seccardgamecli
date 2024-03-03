use serde::{Deserialize, Serialize};
use crate::cards::errors::{ErrorKind, ModelError};
use crate::cards::world::resources::Resources;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FixCost {
    pub min: Resources,
    pub max: Resources,
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
