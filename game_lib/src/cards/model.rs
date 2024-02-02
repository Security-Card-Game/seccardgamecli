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

impl EventCard {
    pub fn empty() -> Card {
        Card::Event(EventCard {
            title: "".to_string(),
            description: "".to_string(),
            action: "".to_string(),
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IncidentCard {
    pub title: String,
    pub description: String,
    pub targets: Vec<String>,
    pub action: String,
}

impl IncidentCard {
    pub fn empty() -> Card {
        Card::Incident(IncidentCard {
            title: "".to_string(),
            description: "".to_string(),
            targets: vec![],
            action: "".to_string(),
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LuckyCard {
    pub title: String,
    pub description: String,
    pub action: String,
}

impl LuckyCard {
    pub fn empty() -> Card {
        Card::Lucky(LuckyCard {
            title: "".to_string(),
            description: "".to_string(),
            action: "".to_string(),
        })
    }
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

impl OopsieCard {
    pub fn empty() -> Card {
        Card::Oopsie(OopsieCard {
            title: "".to_string(),
            description: "".to_string(),
            targets: vec![],
            action: "".to_string(),
            fix_cost: FixCost { min: 0, max: 0 },
        })
    }
}

pub trait CardTrait {
    fn title(&self) -> &String;
    fn description(&self) -> &String;
    fn action(&self) -> &String;

    fn category(&self) -> &str;
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

    fn category(&self) -> &str {
        match self {
            Card::Event(_) => Card::EVENT_CARD,
            Card::Incident(_) => Card::INCIDENT_CARD,
            Card::Oopsie(_) => Card::OOPSIE_CARD,
            Card::Lucky(_) => Card::LUCKY_CARD,
        }
    }
}
