use eframe::epaint::Color32;
use game_lib::cards::model::{Card, EventCard, FixCost, IncidentCard, LuckyCard, OopsieCard};
use uuid::Uuid;

pub struct CardContent {
    pub id: Uuid,
    pub dark_color: Color32,
    pub light_color: Color32,
    pub label: String,
    pub description: String,
    pub action: String,
    pub targets: Option<Vec<String>>,
    pub costs: Option<FixCost>,
}

pub fn to_ui_deck(deck: Vec<Card>) -> Vec<CardContent> {
    let mut ui_deck: Vec<_> = deck
        .iter()
        .map(|c| CardContent::from_card(c))
        .collect();
    ui_deck.reverse();
    ui_deck
}

impl CardContent {
    fn from_card(card: &Card) -> CardContent {
        match card {
            Card::Event(c) => Self::event_card_content(c.clone()),
            Card::Incident(c) => Self::incident_card_content(c.clone()),
            Card::Oopsie(c) => Self::oopsie_card_content(c.clone()),
            Card::Lucky(c) => Self::lucky_card_content(c.clone()),
        }
    }

    fn event_card_content(card: EventCard) -> CardContent {
        let id = Uuid::new_v4();
        CardContent {
            id,
            dark_color: Color32::LIGHT_BLUE,
            light_color: Color32::DARK_BLUE,
            label: card.title,
            description: card.description,
            action: card.action,
            targets: None,
            costs: None,
        }
    }

    fn incident_card_content(card: IncidentCard) -> CardContent {
        let id = Uuid::new_v4();
        CardContent {
            id,
            dark_color: Color32::LIGHT_RED,
            light_color: Color32::DARK_RED,
            label: card.title,
            description: card.description,
            action: card.action,
            targets: Some(card.targets),
            costs: None,
        }
    }

    fn oopsie_card_content(card: OopsieCard) -> CardContent {
        let id = Uuid::new_v4();
        CardContent {
            id,
            dark_color: Color32::YELLOW,
            light_color: Color32::DARK_GRAY,
            label: card.title,
            description: card.description,
            action: card.action,
            targets: Some(card.targets),
            costs: Some(card.fix_cost),
        }
    }

    fn lucky_card_content(card: LuckyCard) -> CardContent {
        let id = Uuid::new_v4();
        CardContent {
            id,
            dark_color: Color32::GREEN,
            light_color: Color32::DARK_GREEN,
            label: card.title,
            description: card.description,
            action: card.action,
            targets: None,
            costs: None,
        }
    }
}
