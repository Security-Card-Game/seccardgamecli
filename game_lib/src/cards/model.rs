use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
#[serde(untagged)]
pub enum Card {
    Event(EventCard),
    Incident(IncidentCard),
    Oopsie(OopsieCard),
    Lucky(LuckyCard),
}

impl Card {
    pub const EVENT_CARD: &'static str = "Event";
    pub const INCIDENT_CARD: &'static str = "Incident";
    pub const OOPSIE_CARD: &'static str = "Oopsie";
    pub const LUCKY_CARD: &'static str = "Lucky";

    pub const CARD_TYPES: [&'static str; 4] = [
        Self::EVENT_CARD,
        Self::INCIDENT_CARD,
        Self::OOPSIE_CARD,
        Self::LUCKY_CARD,
    ];
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EventCard {
    pub title: String,
    pub description: String,
    pub action: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IncidentCard {
    pub title: String,
    pub description: String,
    pub targets: Vec<String>,
    pub action: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LuckyCard {
    pub title: String,
    pub description: String,
    pub action: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FixCost {
    pub min: u8,
    pub max: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OopsieCard {
    pub title: String,
    pub description: String,
    pub targets: Vec<String>,
    pub action: String,
    pub fix_cost: FixCost,
}

pub trait CardTrait {
    fn title(&self) -> &String;
    fn description(&self) -> &String;
    fn action(&self) -> &String;
}

impl CardTrait for Card {
    fn title(&self) -> &String {
        match self {
            Card::Event(card) => &card.title,
            Card::Incident(card) => &card.title,
            Card::Oopsie(card) => &card.title,
            Card::Lucky(card) => &card.title,
        }
    }

    fn description(&self) -> &String {
        match self {
            Card::Event(card) => &card.description,
            Card::Incident(card) => &card.description,
            Card::Oopsie(card) => &card.description,
            Card::Lucky(card) => &card.description,
        }
    }

    fn action(&self) -> &String {
        match self {
            Card::Event(card) => &card.action,
            Card::Incident(card) => &card.action,
            Card::Oopsie(card) => &card.action,
            Card::Lucky(card) => &card.action,
        }
    }
}
