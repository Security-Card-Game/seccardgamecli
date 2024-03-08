use std::rc::Rc;
use eframe::epaint::Color32;
use uuid::Uuid;
use game_lib::cards::properties::effect::Effect;
use game_lib::cards::properties::fix_cost::FixCost;
use game_lib::cards::properties::target::Target;
use game_lib::cards::types::attack::AttackCard;
use game_lib::cards::types::card_model::Card;
use game_lib::cards::types::event::EventCard;
use game_lib::cards::types::lucky::LuckyCard;
use game_lib::cards::types::oopsie::OopsieCard;
use game_lib::world::deck::CardRc;


#[derive(Debug)]
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

impl CardContent {
    pub fn from_card(id: &Uuid, card: CardRc) -> CardContent {

        match &*card {
            Card::Event(c) => Self::event_card_content(id, c.clone()),
            Card::Attack(c) => Self::incident_card_content(id, c.clone()),
            Card::Oopsie(c) => Self::oopsie_card_content(id, c.clone()),
            Card::Lucky(c) => Self::lucky_card_content(id, c.clone()),
        }
    }

    fn event_card_content(id: &Uuid, card: EventCard) -> CardContent {
        CardContent {
            id: id.clone(),
            dark_color: Color32::LIGHT_BLUE,
            light_color: Color32::DARK_BLUE,
            label: card.title.value().to_string(),
            description: card.description.value().to_string(),
            action: Self::effect_to_text(&card.effect),
            targets: None,
            costs: None,
            duration: None,
        }
    }

    fn effect_to_targets(action: &Effect) -> Option<Vec<String>> {
        match action {
            Effect::Incident(_, t) => Some(Self::targets_to_strings(t)),
            Effect::AttackSurface(_, t) => Some(Self::targets_to_strings(t)),
            _ => None
        }
    }


    fn effect_to_text(action: &Effect) -> String {
        match action {
            Effect::Immediate(d) => d.value().to_string(),
            Effect::Incident(d, _) => d.value().to_string(),
            Effect::OnNextFix(d, m) => format!("{} {}", d.value(), m.value()),
            Effect::OnUsingForFix(d, m) => format!("{} {}", d.value(), m.value()),
            Effect::Other(d) => d.value().to_string(),
            Effect::AttackSurface(d, _) => d.value().to_string(),
            Effect::NOP => "".to_string(),
        }
    }

    fn targets_to_strings(targets: &Vec<Target>) -> Vec<String> {
        targets.iter().map(|i| i.value().to_string()).collect()
    }

    fn incident_card_content(id: &Uuid, card: AttackCard) -> CardContent {
        CardContent {
            id: id.clone(),
            dark_color: Color32::LIGHT_RED,
            light_color: Color32::DARK_RED,
            label: card.title.value().to_string(),
            description: card.description.value().to_string(),
            action: Self::effect_to_text(&card.effect),
            targets: Self::effect_to_targets(&card.effect),
            costs: None,
            duration: Some(card.duration.value().unwrap_or(&0).clone()),
        }
    }

    fn oopsie_card_content(id: &Uuid, card: OopsieCard) -> CardContent {
        CardContent {
            id: id.clone(),
            dark_color: Color32::YELLOW,
            light_color: Color32::DARK_GRAY,
            label: card.title.value().to_string(),
            description: card.description.value().to_string(),
            action: Self::effect_to_text(&card.effect),
            targets: Self::effect_to_targets(&card.effect),
            costs: Some(card.fix_cost),
            duration: None,
        }
    }

    fn lucky_card_content(id: &Uuid, card: LuckyCard) -> CardContent {
        CardContent {
            id: id.clone(),
            dark_color: Color32::GREEN,
            light_color: Color32::DARK_GREEN,
            label: card.title.value().to_string(),
            description: card.description.value().to_string(),
            action: Self::effect_to_text(&card.effect),
            targets: None,
            costs: None,
            duration: None,
        }
    }
}
