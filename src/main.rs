use std::env;
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
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        match args[1].as_str() {
            "--reset" | "--restore-defaults" => {
                println!("Resetting Hypremoji configuration...");

                if let Err(e) = utils::reset_config() {
                    eprintln!("Error resetting configuration: {}", e);
                    std::process::exit(1);
                } else {
                    println!("Configuration reset successfully!");
                    std::process::exit(0);
                }
            }
            _ => {
                eprintln!("Unknown option: {}", args[1]);
                eprintln!("Usage: hypremoji [--reset]");
                std::process::exit(1);
            }
        }
    }

    let cb_manager = utils::get_clipboard_manager();

    let app = Application::builder()
        .application_id("dev.musagy.hypremoji")
        .build();

    app.connect_startup(|_| {
        if let Err(e) = load_css() {
            eprintln!("Error loading CSS: {}", e);
        }
    });

    let cb_manager_clone = cb_manager.clone();
    app.connect_activate(move |app| {
        build_ui(app, cb_manager_clone.clone());
    });
    app.run();

    cb_manager.send_emoji_to_focused_window();
}
