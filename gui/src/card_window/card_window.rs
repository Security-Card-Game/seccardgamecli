use eframe::epaint::FontFamily;
use egui::{Context, Label, Pos2, RichText, Ui, Vec2, WidgetText, Window};
use rand::Rng;
use crate::actions::command::Command;
use crate::card_window::card_view_model::{CardContent, CardMarker};


pub struct CardWindow<'a> {
    max_size: Vec2,
    min_size: Vec2,
    content: &'a CardContent,
}

pub fn display_card<F>(
    card: &CardContent,
    command_callback: &mut F,
    ctx: &Context,
    ui: &mut Ui,
)  where F: FnMut(Command)
{
    let window = CardWindow {
        max_size: Vec2::new(200.0, 400.0),
        min_size: Vec2::new(150.0, 300.0),
        content: card,
    };

    create_window(window, command_callback, ctx, ui)
}

fn create_window<F>(
    data: CardWindow,
    command_callback: &mut F,
    ctx: &Context,
    ui: &mut Ui,
) where F: FnMut(Command) {
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
        .show(ctx, |ui| {
            create_card_window(command_callback, card, ui)
        });
}

fn create_card_window<F>(cmd_callback: &mut F, card: &CardContent, ui: &mut Ui)
where F: FnMut(Command)   {
    ui.vertical(|ui| {
        add_header(cmd_callback, &card, ui);
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
                let content = format!("{} to {} resources", cost.min.value(), cost.max.value());
                add_explanation("Fix:      ", content.as_str(), ui);
            }
        };
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
    });
}


fn add_header<F>(cmd_callback: &mut F, card: &&CardContent, ui: &mut Ui)
where F: FnMut(Command) {
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
            if ui.button("X").clicked() {
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
