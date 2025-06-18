use gtk::prelude::*;
use gtk::Application;
use gtk::{ApplicationWindow, Box};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use utils::find_emoji_by_name;

mod category;
mod load_styles;
mod ui;
mod utils;

use category::Category;
use load_styles::load_css;
use ui::create_category_nav;
use ui::create_emoji_grid_section;
use ui::create_search_section;
use utils::load_emoji_for_category;

fn main() {
    let app = Application::builder()
        .application_id("dev.musagy.HyprEmoji")
        .build();

    app.connect_startup(|_| {
        if let Err(e) = load_css() {
            eprintln!("Error al cargar el CSS: {}", e);
        }
        if let Err(w) = find_emoji_by_name("ship") {
            eprintln!("Error al buscar emojis: {}", w);
        }
    });
    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("HyprEmoji")
        .default_width(284)
        .default_height(340)
        .build();

    let window_ref = Rc::new(RefCell::new(window.clone()));

    let side_margin = 12;
    let vertical_margin = 10;

    let main_box = Box::new(gtk::Orientation::Vertical, 0);
    window.set_child(Some(&main_box));

    let all_emojis_by_category: Rc<RefCell<HashMap<Category, Vec<String>>>> = {
        match load_emoji_for_category() {
            Ok(map) => Rc::new(RefCell::new(map)),
            Err(e) => {
                eprintln!("Error al cargar emojis: {}", e);
                Rc::new(RefCell::new(HashMap::new()))
            }
        }
    };
    all_emojis_by_category
        .borrow()
        .get(&Category::SmileysAndEmotion)
        .expect("No se encontraron emojis en la categoría Smileys & Emotion");

    // Crear sección de búsqueda
    let search_section = create_search_section(side_margin);
    main_box.append(&search_section);

    // Crear barra de navegación de categorías
    let selected_category = Rc::new(RefCell::new(Category::SmileysAndEmotion));

    // Crear cuadrícula de emojis
    let (emoji_grid_widget, emoji_flowbox_ref) = create_emoji_grid_section(
        side_margin,
        vertical_margin,
        selected_category.clone(),
        all_emojis_by_category.clone(),
        window_ref.clone(),
    );

    let category_nav = create_category_nav(
        side_margin,
        vertical_margin,
        selected_category.clone(),
        all_emojis_by_category.clone(),
        emoji_flowbox_ref,
    );
    main_box.append(&category_nav);
    main_box.append(&emoji_grid_widget);

    window.present();
}
