use serde::{Deserialize, Serialize};
use crate::world::resources::Resources;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub enum FixModifier {
    Increase(Resources),
    Decrease(Resources),
}

impl FixModifier {
    pub fn value(&self) -> isize {
        match self {
            FixModifier::Increase(r) => r.value().clone() as isize,
            FixModifier::Decrease(r) => -1 * r.value().clone() as isize
        }
    }
}

