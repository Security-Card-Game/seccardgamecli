use egui::{Context, Pos2, RichText, Ui, Vec2, Window};
use rand::Rng;
use uuid::Uuid;
use crate::card::CardContent;

pub struct CardWindow<'a> {
    max_size: Vec2,
    min_size: Vec2,
    content: &'a CardContent
}


pub fn display_card<F>(card: &CardContent, close_callback: F, ctx: &Context, ui: &mut Ui)
    where F: FnMut(Uuid) -> () {
    let window = CardWindow {
        max_size: Vec2::new(200.0, 400.0),
        min_size: Vec2::new(150.0, 300.0),
        content: card
    };
    create_window(window, close_callback, ctx, ui)
}

fn create_window<F>(data: CardWindow, close_callback: F, ctx: &Context, ui: &mut Ui)
    where F: FnMut(Uuid) -> () {
    let card = data.content;
    let area= ui.available_size();
    let mut rng = rand::thread_rng();
    let offset_x = rng.gen_range(-20.0..20.0);
    let offset_y = rng.gen_range(-20.0..20.0);
    let new_pos = Pos2::new(area.x/3.0 + offset_x, area.y/3.0 + offset_y);
    Window::new(card.id.to_string())
        .title_bar(false)
        .resizable(false)
        .collapsible(false)
        .default_pos(new_pos)
        .max_size(data.max_size)
        .min_size(data.min_size)
        .show(ctx, |ui| {
            create_card_window(close_callback, card, ui)
        });
}

fn create_card_window<F>(mut close_callback: F, card: &CardContent, ui: &mut Ui)
    where F: FnMut(Uuid) -> () {
    ui.vertical(|ui| {
        ui.horizontal(|ui| {
            let header_color = if ui.visuals().dark_mode {
                card.dark_color
            } else {
                card.light_color
            };
            let header = RichText::new(&card.label).color(header_color).heading();
            ui.label(header);
            if ui.button("X").clicked() {
                close_callback(card.id);
            }
        });
        ui.add_space(5.0);
        ui.label(&card.description);
        ui.add_space(2.0);
        let name = RichText::new("Action: ").strong();
        ui.label(name);
        ui.label(&card.action);
        match &card.targets {
            None => {}
            Some(targets) => {
                let name = RichText::new("Targets: ").strong();
                ui.label(name);
                let list = targets.join(", ");
                ui.label(list);
            }
        }
        ui.add_space(2.0);
        match &card.costs {
            None => {}
            Some(cost) => {
                let name = RichText::new("Cost to fix: ").strong();
                ui.label(name);
                ui.label(format!("{} to {}", cost.min, cost.max));
            }
        };
    });
}

