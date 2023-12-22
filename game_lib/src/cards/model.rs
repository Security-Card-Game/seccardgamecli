use serde::{Deserialize, Serialize};

pub const CARD_TYPES: &[&str; 3] = &["Magic", "Trap", "Monster"];

#[derive(Serialize, Deserialize, Debug)]
pub struct Card {
    pub card_type: String,
    pub title: String,
    pub description: String,
    pub cost: u8,
}

