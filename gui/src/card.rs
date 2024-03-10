use eframe::epaint::Color32;
use uuid::Uuid;

use game_lib::cards::properties::effect::Effect;
use game_lib::cards::properties::fix_cost::FixCost;
use game_lib::cards::properties::target::Target;
use game_lib::cards::types::attack::AttackCard;
use game_lib::cards::types::card_model::{Card, CardTrait};
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
    pub can_be_activated: bool
}

impl CardContent {
    pub fn new(
        id: Uuid,
        dark_color: Color32,
        light_color: Color32,
        card: Card,
        costs: Option<FixCost>,
        duration: Option<usize>,
    ) -> CardContent {
        CardContent {
            id,
            dark_color,
            light_color,
            label: card.title().value().to_string(),
            description: card.description().value().to_string(),
            action: Self::effect_to_text(&card.effect()),
            targets: Self::effect_to_targets(&card.effect()),
            costs,
            duration,
            can_be_activated: Self::can_effect_be_activated(&card.effect())
        }
    }

    pub fn from_card(id: &Uuid, card: CardRc) -> CardContent {
        match &*card {
            Card::Event(c) => Self::event_card_content(id, c.clone()),
            Card::Attack(c) => Self::incident_card_content(id, c.clone()),
            Card::Oopsie(c) => Self::oopsie_card_content(id, c.clone()),
            Card::Lucky(c) => Self::lucky_card_content(id, c.clone()),
        }
    }

    fn event_card_content(id: &Uuid, card: EventCard) -> CardContent {
        Self::new(
            id.clone(),
            Color32::LIGHT_BLUE,
            Color32::DARK_BLUE,
            Card::Event(card),
            None,
            None,
        )
    }

    fn can_effect_be_activated(effect: &Effect) -> bool {
        match effect {
            Effect::Immediate(_)
            | Effect::AttackSurface(_, _)
            | Effect::Incident(_, _)
            | Effect::OnNextFix(_, _)
            | Effect::Other(_)
            | Effect::NOP => false,
            Effect::OnUsingForFix(_, _) => true,
        }
    }

    fn effect_to_targets(action: &Effect) -> Option<Vec<String>> {
        match action {
            Effect::Incident(_, t) => Some(Self::targets_to_strings(t)),
            Effect::AttackSurface(_, t) => Some(Self::targets_to_strings(t)),
            _ => None,
        }
    }

    fn effect_to_text(action: &Effect) -> String {
        match action {
            Effect::Immediate(d) | Effect::Other(d) => d.value().to_string(),
            Effect::OnNextFix(d, _)
            | Effect::Incident(d, _)
            | Effect::OnUsingForFix(d, _)
            | Effect::AttackSurface(d, _) => d.value().to_string(),
            Effect::NOP => "".to_string(),
        }
    }

    fn targets_to_strings(targets: &Vec<Target>) -> Vec<String> {
        targets.iter().map(|i| i.value().to_string()).collect()
    }

    fn incident_card_content(id: &Uuid, card: AttackCard) -> CardContent {
        let duration = card.duration.value().unwrap_or(&0).clone();
        Self::new(
            id.clone(),
            Color32::LIGHT_RED,
            Color32::DARK_RED,
            Card::Attack(card),
            None,
            Some(duration),
        )
    }


    fn oopsie_card_content(id: &Uuid, card: OopsieCard) -> CardContent {
        let fix_cost = &card.fix_cost.clone();
        Self::new(
            id.clone(),
            Color32::YELLOW,
            Color32::DARK_GRAY,
            Card::Oopsie(card),
            Some(fix_cost.clone()),
            None,
        )
    }

    fn lucky_card_content(id: &Uuid, card: LuckyCard) -> CardContent {
        Self::new(
            id.clone(),
            Color32::GREEN,
            Color32::DARK_GREEN,
            Card::Lucky(card),
            None,
            None,
        )
    }
}
