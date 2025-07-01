use gtk::prelude::*;
use gtk::Application;

mod category;
mod load_styles;
mod services;
mod ui;
mod utils;

use crate::ui::build_ui;
use load_styles::load_css;

fn main() {
    let cb_manager = utils::get_clipboard_manager();

    let app = Application::builder()
        .application_id("dev.musagy.hypremoji")
        .build();

    app.connect_startup(|_| {
        if let Err(e) = load_css() {
            eprintln!("Error al cargar el CSS: {}", e);
        }
    });

    let cb_manager_clone = cb_manager.clone();
    app.connect_activate(move |app| {
        build_ui(app, cb_manager_clone.clone());
    });
    app.run();

    cb_manager.send_emoji_to_focused_window();
}
