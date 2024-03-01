use eframe::epaint::FontFamily;
use egui::{Context, Label, Pos2, RichText, Ui, Vec2, WidgetText, Window};
use rand::Rng;
use uuid::Uuid;

use crate::card::CardContent;

pub struct CardWindow<'a> {
    max_size: Vec2,
    min_size: Vec2,
    content: &'a CardContent,
}

pub fn display_card<F>(card: &CardContent, close_callback: F, ctx: &Context, ui: &mut Ui)
where
    F: FnMut(Uuid) -> (),
{
    let window = CardWindow {
        max_size: Vec2::new(200.0, 400.0),
        min_size: Vec2::new(150.0, 300.0),
        content: card,
    };
    create_window(window, close_callback, ctx, ui)
}

fn create_window<F>(data: CardWindow, close_callback: F, ctx: &Context, ui: &mut Ui)
where
    F: FnMut(Uuid) -> (),
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
        .show(ctx, |ui| create_card_window(close_callback, card, ui));
}

fn create_card_window<F>(close_callback: F, card: &CardContent, ui: &mut Ui)
where
    F: FnMut(Uuid) -> (),
{
    ui.vertical(|ui| {
        add_header(close_callback, &card, ui);
        ui.add_space(5.0);
        card_label(&card.description, ui);
        ui.add_space(5.0);
        add_explanation("Action:   ", &card.action.as_str(), ui);
        match &card.duration {
            None => {}

            Some(duration) => {
                let content = format!("{} rounds", duration);
                add_explanation("Duration: ", content.as_str(), ui);
            }
        }
        match &card.targets {
            None => {}

            Some(targets) => {
                let content = targets.join(", ");
                add_explanation("Targets:  ", content.as_str(), ui);
            }
        }
        ui.add_space(2.0);
        match &card.costs {
            None => {}
            Some(cost) => {
                let content = format!("{} to {} resources", cost.min, cost.max);
                add_explanation("Fix:      ", content.as_str(), ui);
            }
        };
    });
}

fn add_header<F>(mut close_callback: F, card: &&CardContent, ui: &mut Ui)
where
    F: FnMut(Uuid) -> (),
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
        if ui.button("X").clicked() {
            close_callback(card.id);
        }
    });
}

fn card_label(text: impl Into<WidgetText>, ui: &mut Ui) {
    ui.add(Label::new(text).wrap(true).selectable(false));
}

fn add_explanation(topic: &str, content: &str, ui: &mut Ui) {
    ui.horizontal(|ui| {
        let topic_text = RichText::new(topic).strong().family(FontFamily::Monospace);
        card_label(topic_text, ui);
        card_label(content, ui);
    });
}
