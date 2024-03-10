use serde::{Deserialize, Serialize};
use crate::cards::properties::description::Description;
use crate::cards::properties::effect::Effect;
use crate::cards::properties::title::Title;
use crate::cards::types::attack::AttackCard;
use crate::cards::types::event::EventCard;
use crate::cards::types::lucky::LuckyCard;
use crate::cards::types::oopsie::OopsieCard;


#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum Card {
    Event(EventCard),
    Attack(AttackCard),
    Oopsie(OopsieCard),
    Lucky(LuckyCard),
}

impl Card {
    pub const EVENT_CARD: &'static str = "Event";
    pub const ATTACK_CARD: &'static str = "Attack";
    pub const OOPSIE_CARD: &'static str = "Oopsie";
    pub const LUCKY_CARD: &'static str = "Lucky";

    pub const CARD_TYPES: [&'static str; 4] = [
        Self::ATTACK_CARD,
        Self::EVENT_CARD,
        Self::LUCKY_CARD,
        Self::OOPSIE_CARD,
    ];
}

pub trait CardTrait {
    fn title(&self) -> &Title;
    fn description(&self) -> &Description;
    fn effect(&self) -> &Effect;
    fn category(&self) -> &str;

    fn as_enum(&self) -> Card;
}

impl CardTrait for Card {
    fn title(&self) -> &Title {
        match self {
            Card::Event(card) => &card.title,
            Card::Attack(card) => &card.title,
            Card::Oopsie(card) => &card.title,
            Card::Lucky(card) => &card.title,
        }
    }

    fn description(&self) -> &Description {
        match self {
            Card::Event(card) => &card.description,
            Card::Attack(card) => &card.description,
            Card::Oopsie(card) => &card.description,
            Card::Lucky(card) => &card.description,
        }
    }

    fn effect(&self) -> &Effect {
        match self {
            Card::Event(card) => &card.effect,
            Card::Attack(card) => &card.effect,
            Card::Oopsie(card) => &card.effect,
            Card::Lucky(card) => &card.effect,
        }
    }

    fn category(&self) -> &str {
        match self {
            Card::Event(_) => Card::EVENT_CARD,
            Card::Attack(_) => Card::ATTACK_CARD,
            Card::Oopsie(_) => Card::OOPSIE_CARD,
            Card::Lucky(_) => Card::LUCKY_CARD,
        }
    }

    fn as_enum(&self) -> Card {
        match self {
            Card::Event(_) => EventCard::empty(),
            Card::Attack(_) => AttackCard::empty(),
            Card::Oopsie(_) => OopsieCard::empty(),
            Card::Lucky(_) => LuckyCard::empty(),
        }
    }
}
