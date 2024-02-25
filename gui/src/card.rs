use eframe::epaint::Color32;
use uuid::Uuid;
use game_lib::cards::model::{EventCard, FixCost, IncidentCard, LuckyCard, OopsieCard};

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


pub fn event_card_content(card: EventCard) -> CardContent{
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

pub fn incident_card_content(card: IncidentCard) -> CardContent {
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

pub fn oopsie_card_content(card: OopsieCard) -> CardContent {
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

pub fn lucky_card_content(card: LuckyCard) -> CardContent {
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