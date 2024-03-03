use serde::{Deserialize, Serialize};

use crate::cards::properties::card_content::{Action, Description, Duration, FixCost, Target, Title};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
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
    pub title: Title,
    pub description: Description,
    pub action: Action,
}

impl EventCard {
    pub fn empty() -> Card {
        Card::Event(EventCard {
            title: Title::empty(),
            description: Description::empty(),
            action: Action::NOP,
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IncidentCard {
    pub title: Title,
    pub description: Description,
    pub targets: Vec<Target>,
    pub action: Action,
    #[serde(default)]
    pub duration: Duration,
}

impl IncidentCard {
    pub fn empty() -> Card {
        Card::Incident(IncidentCard {
            title: Title::empty(),
            description: Description::empty(),
            targets: vec![],
            action: Action::default(),
            duration: Duration::default(),
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LuckyCard {
    pub title: Title,
    pub description: Description,
    pub action: Action,
}

impl LuckyCard {
    pub fn empty() -> Card {
        Card::Lucky(LuckyCard {
            title: Title::empty(),
            description: Description::empty(),
            action: Action::default(),
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OopsieCard {
    pub title: Title,
    pub description: Description,
    pub targets: Vec<Target>,
    pub action: Action,
    pub fix_cost: FixCost,
}

impl OopsieCard {
    pub fn empty() -> Card {
        Card::Oopsie(OopsieCard {
            title: Title::empty(),
            description: Description::empty(),
            targets: vec![],
            action: Action::default(),
            fix_cost: FixCost::default(),
        })
    }
}

pub trait CardTrait {
    fn title(&self) -> &Title;
    fn description(&self) -> &Description;
    fn action(&self) -> &Action;
    fn category(&self) -> &str;

    fn as_enum(&self) -> Card;
}

impl CardTrait for Card {
    fn title(&self) -> &Title {
        match self {
            Card::Event(card) => &card.title,
            Card::Incident(card) => &card.title,
            Card::Oopsie(card) => &card.title,
            Card::Lucky(card) => &card.title,
        }
    }

    fn description(&self) -> &Description {
        match self {
            Card::Event(card) => &card.description,
            Card::Incident(card) => &card.description,
            Card::Oopsie(card) => &card.description,
            Card::Lucky(card) => &card.description,
        }
    }

    fn action(&self) -> &Action {
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

    fn as_enum(&self) -> Card {
        match self {
            Card::Event(_) => EventCard::empty(),
            Card::Incident(_) => IncidentCard::empty(),
            Card::Oopsie(_) => OopsieCard::empty(),
            Card::Lucky(_) => LuckyCard::empty(),
        }
    }
}
