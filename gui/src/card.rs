use eframe::epaint::Color32;
use uuid::Uuid;
use game_lib::cards::properties::effect::Effect;
use game_lib::cards::properties::fix_cost::FixCost;
use game_lib::cards::properties::target::Target;
use game_lib::cards::types::attack::IncidentCard;
use game_lib::cards::types::card_model::Card;
use game_lib::cards::types::event::EventCard;
use game_lib::cards::types::lucky::LuckyCard;
use game_lib::cards::types::oopsie::OopsieCard;


pub struct CardContent {
    pub id: Uuid,
    pub dark_color: Color32,
    pub light_color: Color32,
    pub label: String,
    pub description: String,
    pub action: String,
    pub targets: Option<Vec<String>>,
    pub costs: Option<FixCost>,
    pub duration: Option<usize>,
}

pub fn to_ui_deck(deck: Vec<Card>) -> Vec<CardContent> {
    let mut ui_deck: Vec<_> = deck.iter().map(|c| CardContent::from_card(c)).collect();
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
            label: card.title.value().to_string(),
            description: card.description.value().to_string(),
            action: Self::action_to_text(card.action),
            targets: None,
            costs: None,
            duration: None,
        }
    }

    fn action_to_text(action: Effect) -> String {
        match action {
            Effect::Immediate(d) => d.value().to_string(),
            Effect::OnTargetAvailable(d) => d.value().to_string(),
            Effect::OnNextFix(d, m) => format!("{} {}", d.value(), m.value()),
            Effect::OnUsingForFix(d, m) => format!("{} {}", d.value(), m.value()),
            Effect::Other(d) => d.value().to_string(),
            Effect::NOP => "".to_string(),
        }
    }

    fn targets_to_strings(targets: Vec<Target>) -> Vec<String> {
        targets.iter().map(|i| i.value().to_string()).collect()
    }

    fn incident_card_content(card: IncidentCard) -> CardContent {
        let id = Uuid::new_v4();
        CardContent {
            id,
            dark_color: Color32::LIGHT_RED,
            light_color: Color32::DARK_RED,
            label: card.title.value().to_string(),
            description: card.description.value().to_string(),
            action: Self::action_to_text(card.action),
            targets: Some(Self::targets_to_strings(card.targets)),
            costs: None,
            duration: Some(card.duration.value().unwrap_or(&0).clone()),
        }
    }

    fn oopsie_card_content(card: OopsieCard) -> CardContent {
        let id = Uuid::new_v4();
        CardContent {
            id,
            dark_color: Color32::YELLOW,
            light_color: Color32::DARK_GRAY,
            label: card.title.value().to_string(),
            description: card.description.value().to_string(),
            action: Self::action_to_text(card.action),
            targets: Some(Self::targets_to_strings(card.targets)),
            costs: Some(card.fix_cost),
            duration: None,
        }
    }

    fn lucky_card_content(card: LuckyCard) -> CardContent {
        let id = Uuid::new_v4();
        CardContent {
            id,
            dark_color: Color32::GREEN,
            light_color: Color32::DARK_GREEN,
            label: card.title.value().to_string(),
            description: card.description.value().to_string(),
            action: Self::action_to_text(card.action),
            targets: None,
            costs: None,
            duration: None,
        }
    }
}
