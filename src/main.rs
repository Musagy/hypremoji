use gtk::prelude::*;
use gtk::Application;
use std::cell::RefCell;
use std::rc::Rc;

mod category;
mod load_styles;
mod services;
mod ui;
mod utils;

use crate::ui::build_ui;
use load_styles::load_css;

fn main() {
    let emoji_typing_service = utils::get_clipboard_manager();

    let app = Application::builder()
        .application_id("dev.musagy.hypremoji")
        .build();

    app.connect_startup(|_| {
        if let Err(e) = load_css() {
            eprintln!("Error al cargar el CSS: {}", e);
        }
    });

    let chosen_emoji = Rc::new(RefCell::new(None));

    // chosen_emoji.borrow_mut().replace("😀".to_string());

    let emoji_for_later = chosen_emoji.clone();
    app.connect_activate(move |app| {
        build_ui(app, emoji_for_later.clone());
    });
    app.run();

    let emoji = chosen_emoji.borrow().clone();
    if let Some(emoji) = emoji {
        emoji_typing_service.send_emoji_to_focused_window(&emoji);
    }
}
