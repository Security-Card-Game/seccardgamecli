use serde::{Deserialize, Serialize};
use crate::cards::properties::description::Description;
use crate::cards::properties::effect::Effect;
use crate::cards::properties::title::Title;
use crate::cards::types::attack::IncidentCard;
use crate::cards::types::event::EventCard;
use crate::cards::types::lucky::LuckyCard;
use crate::cards::types::oopsie::OopsieCard;


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

pub trait CardTrait {
    fn title(&self) -> &Title;
    fn description(&self) -> &Description;
    fn action(&self) -> &Effect;
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

    fn action(&self) -> &Effect {
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
