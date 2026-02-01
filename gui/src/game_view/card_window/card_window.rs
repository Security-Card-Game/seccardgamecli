use crate::game_view::actions::command::Command;
use crate::game_view::card_window::card_view_model::{CardContent, CardMarker};
use eframe::epaint::FontFamily;
use egui::{Context, Label, Pos2, RichText, Ui, Vec2, WidgetText, Window};
use game_lib::cards::properties::incident_impact::IncidentImpact;
use rand::Rng;

pub struct CardWindow<'a> {
    max_size: Vec2,
    min_size: Vec2,
    content: &'a CardContent,
}

pub fn display_card<F>(card: &CardContent, command_callback: &mut F, ctx: &Context, ui: &mut Ui)
where
    F: FnMut(Command),
{
    let window = CardWindow {
        max_size: Vec2::new(200.0, 400.0),
        min_size: Vec2::new(150.0, 300.0),
        content: card,
    };

    create_window(window, command_callback, ctx, ui)
}

fn create_window<F>(data: CardWindow, command_callback: &mut F, ctx: &Context, ui: &mut Ui)
where
    F: FnMut(Command),
{
    let card = data.content;
    let area = ui.available_size();
    let mut rng = rand::thread_rng();
    let offset_x = rng.gen_range(-20.0..20.0);
    let offset_y = rng.gen_range(-20.0..20.0);
    let new_pos = Pos2::new(area.x / 3.0 + offset_x, area.y / 3.0 + offset_y);
    Window::new(card.id.to_string())
        .title_bar(false)
        .resizable(false)
        .collapsible(false)
        .default_pos(new_pos)
        .max_size(data.max_size)
        .min_size(data.min_size)
        .show(ctx, |ui| create_card_window(command_callback, card, ui));
}

fn create_card_window<F>(cmd_callback: &mut F, card: &CardContent, ui: &mut Ui)
where
    F: FnMut(Command),
{
    ui.vertical(|ui| {
        add_header(cmd_callback, &card, ui);

        ui.add_space(5.0);

        card_label(&card.description, ui);

        ui.add_space(5.0);

        add_action(&card, ui);
        add_duration(&card, ui);
        add_targets(&card, ui);

        ui.add_space(2.0);

        add_fix_costs(&card, ui);
        // this is either fix costs or incident impact
        add_incident_impact(&card, ui);

        ui.add_space(1.0);

        add_activation_button(cmd_callback, card, ui);
    });
}

fn add_activation_button<F>(cmd_callback: &mut F, card: &CardContent, ui: &mut Ui)
where
    F: FnMut(Command),
{
    if card.can_be_activated {
        let label = match card.card_marker {
            CardMarker::MarkedForUse => "Do not use",
            CardMarker::None => "Use",
        };
        if ui.button(label).clicked() {
            match card.card_marker {
                CardMarker::MarkedForUse => cmd_callback(Command::DeactivateCard(card.id)),
                CardMarker::None => cmd_callback(Command::ActivateCard(card.id)),
            }
        }
    }
}

fn add_incident_impact(card: &&CardContent, ui: &mut Ui) {
    match &card.incident_impact {
        None => {}
        Some(impact) => {
            let content = match impact {
                IncidentImpact::PartOfRevenue(poh) => {
                    format!("Pay {} of your revenue during this incident", poh)
                }
                IncidentImpact::Fixed(f) => format!("Pay {} resources immediately", f.value()),
            };
            add_explanation("Impact:   ", content.as_str(), ui);
        }
    }
}

fn add_fix_costs(card: &&CardContent, ui: &mut Ui) {
    match &card.costs {
        None => {}
        Some(cost) => {
            let content = format!("{} to {} resources", cost.min.value(), cost.max.value());
            add_explanation("Fix:      ", content.as_str(), ui);
        }
    };
}

fn add_action(card: &&CardContent, ui: &mut Ui) {
    add_explanation("Action:   ", &card.action.as_str(), ui);
}

fn add_duration(card: &&CardContent, ui: &mut Ui) {
    match &card.duration {
        None => {}
        Some(duration) => {
            let content = format!("{} rounds", duration);
            add_explanation("Duration: ", content.as_str(), ui);
        }
    }
}

fn add_targets(card: &&CardContent, ui: &mut Ui) {
    match &card.targets {
        None => {}

        Some(targets) => {
            let content = targets.join(", ");
            add_explanation("Targets:  ", content.as_str(), ui);
        }
    }
}

fn add_header<F>(cmd_callback: &mut F, card: &&CardContent, ui: &mut Ui)
where
    F: FnMut(Command),
{
    ui.horizontal(|ui| {
        let header_color = if ui.visuals().dark_mode {
            card.dark_color
        } else {
            card.light_color
        };

        let header = RichText::new(&card.label).color(header_color).heading();
        card_label(header, ui);
        let available = ui.available_rect_before_wrap().width();
        ui.add_space(available + 20.0);
        if card.can_be_closed {
            if ui.button(card.close_label.clone()).clicked() {
                cmd_callback(Command::CloseCard(card.id));
            }
        }
    });
}

fn card_label(text: impl Into<WidgetText>, ui: &mut Ui) {
    ui.add(Label::new(text).wrap().selectable(false));
}

fn add_explanation(topic: &str, content: &str, ui: &mut Ui) {
    ui.horizontal(|ui| {
        let topic_text = RichText::new(topic).strong().family(FontFamily::Monospace);
        card_label(topic_text, ui);
        card_label(content, ui);
    });
}
