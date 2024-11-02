use std::ops::Add;

use eframe::epaint::Color32;
use uuid::Uuid;

use game_lib::cards::properties::effect::Effect;
use game_lib::cards::properties::fix_cost::FixCost;
use game_lib::cards::properties::fix_modifier::FixModifier;
use game_lib::cards::properties::target::Target;
use game_lib::cards::types::attack::AttackCard;
use game_lib::cards::types::card_model::{Card, CardTrait};
use game_lib::cards::types::evaluation::EvaluationCard;
use game_lib::cards::types::event::EventCard;
use game_lib::cards::types::lucky::LuckyCard;
use game_lib::cards::types::oopsie::OopsieCard;
use game_lib::world::deck::CardRc;
use game_lib::world::resource_fix_multiplier::ResourceFixMultiplier;

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
    pub can_be_activated: bool,
    pub can_be_closed: bool,
    pub card_marker: CardMarker,
}

#[derive(Clone, Debug)]
pub enum CardMarker {
    MarkedForUse,
    None,
}

impl CardContent {
    pub fn new(
        id: Uuid,
        dark_color: Color32,
        light_color: Color32,
        card: Card,
        costs: Option<FixCost>,
        duration: Option<usize>,
        can_be_closed: bool,
        multiplier: ResourceFixMultiplier,
    ) -> CardContent {
        let actual_costs = match costs {
            None => None,
            Some(c) => Some(c * &multiplier),
        };

        CardContent {
            id,
            dark_color,
            light_color,
            label: card.title().value().to_string(),
            description: card.description().value().to_string(),
            action: Self::effect_to_text(&card.effect(), &multiplier),
            targets: Self::effect_to_targets(&card.effect()),
            costs: actual_costs,
            duration,
            can_be_activated: Self::can_effect_be_activated(&card.effect()),
            can_be_closed,
            card_marker: CardMarker::None,
        }
    }

    pub fn from_card(
        id: &Uuid,
        card: CardRc,
        is_active: bool,
        multiplier: ResourceFixMultiplier,
    ) -> CardContent {
        let mut card_view_model = match &*card {
            Card::Event(c) => Self::event_card_content(id, c.clone(), multiplier),
            Card::Attack(c) => Self::incident_card_content(id, c.clone(), multiplier),
            Card::Oopsie(c) => Self::oopsie_card_content(id, c.clone(), multiplier),
            Card::Lucky(c) => Self::lucky_card_content(id, c.clone(), multiplier),
            Card::Evaluation(c) => Self::evaluation_card_content(id, c.clone(), multiplier),
        };

        card_view_model.card_marker = if is_active {
            CardMarker::MarkedForUse
        } else {
            CardMarker::None
        };

        card_view_model
    }

    fn event_card_content(
        id: &Uuid,
        card: EventCard,
        multiplier: ResourceFixMultiplier,
    ) -> CardContent {
        let can_be_closed = card.is_closeable();
        Self::new(
            id.clone(),
            Color32::LIGHT_BLUE,
            Color32::DARK_BLUE,
            Card::Event(card),
            None,
            None,
            can_be_closed,
            multiplier,
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

    fn effect_to_text(action: &Effect, multiplier: &ResourceFixMultiplier) -> String {
        match action {
            Effect::Immediate(d) | Effect::Other(d) => d.value().to_string(),
            Effect::OnNextFix(_d, m) => Self::modifier_to_text(m, multiplier).add(" on next fix."),
            Effect::OnUsingForFix(_d, m) => {
                Self::modifier_to_text(m, multiplier).add(" on use for a fix.")
            }
            Effect::Incident(d, _) | Effect::AttackSurface(d, _) => d.value().to_string(),
            Effect::NOP => "".to_string(),
        }
    }

    fn modifier_to_text(fix_modifier: &FixModifier, multiplier: &ResourceFixMultiplier) -> String {
        match fix_modifier {
            FixModifier::Increase(r) => format!(
                "Increase fix cost by {} resources",
                (r.clone() * multiplier).value()
            ),
            FixModifier::Decrease(r) => format!(
                "Decrease fix cost by {} resources",
                (r.clone() * multiplier).value()
            ),
        }
    }

    fn targets_to_strings(targets: &Vec<Target>) -> Vec<String> {
        targets.iter().map(|i| i.value().to_string()).collect()
    }

    fn incident_card_content(
        id: &Uuid,
        card: AttackCard,
        multiplier: ResourceFixMultiplier,
    ) -> CardContent {
        let duration = card.duration.value().unwrap_or(&0).clone();
        Self::new(
            id.clone(),
            Color32::LIGHT_RED,
            Color32::DARK_RED,
            Card::Attack(card),
            None,
            Some(duration),
            true,
            multiplier,
        )
    }

    fn oopsie_card_content(
        id: &Uuid,
        card: OopsieCard,
        multiplier: ResourceFixMultiplier,
    ) -> CardContent {
        let fix_cost = &card.fix_cost.clone();
        Self::new(
            id.clone(),
            Color32::YELLOW,
            Color32::DARK_GRAY,
            Card::Oopsie(card),
            Some(fix_cost.clone()),
            None,
            true,
            multiplier,
        )
    }

    fn lucky_card_content(
        id: &Uuid,
        card: LuckyCard,
        multiplier: ResourceFixMultiplier,
    ) -> CardContent {
        let can_be_closed = card.is_closeable();
        Self::new(
            id.clone(),
            Color32::GREEN,
            Color32::DARK_GREEN,
            Card::Lucky(card),
            None,
            None,
            can_be_closed,
            multiplier,
        )
    }

    fn evaluation_card_content(
        id: &Uuid,
        card: EvaluationCard,
        multiplier: ResourceFixMultiplier,
    ) -> CardContent {
        let can_be_closed = card.is_closeable();
        Self::new(
            id.clone(),
            Color32::GREEN,
            Color32::DARK_GREEN,
            Card::Evaluation(card),
            None,
            None,
            can_be_closed,
            multiplier,
        )
    }
}
