use gtk::{gdk::Key, prelude::WidgetExt, EventControllerKey};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

use gtk::{
    prelude::{BoxExt, GtkWindowExt},
    Application, ApplicationWindow, Box as BoxGtk, Entry,
};

use crate::{
    category::Category,
    services::get_search_service,
    ui::{create_category_nav, create_emoji_grid_section, create_search_section},
    utils::load_emoji_for_category,
};

pub fn build_ui(app: &Application, chosen_emoji: Rc<RefCell<Option<String>>>) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("HyprEmoji")
        .default_width(284)
        .default_height(340)
        .build();

    let search_input_rc = Rc::new(RefCell::new(Entry::new()));

    let window_ref = Rc::new(RefCell::new(window.clone()));

    let side_margin = 12;
    let vertical_margin = 10;

    let main_box = BoxGtk::new(gtk::Orientation::Vertical, 0);
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

    // Crear barra de navegación de categorías
    let selected_category = Rc::new(RefCell::new(Category::SmileysAndEmotion));

    // Crear cuadrícula de emojis
    let (emoji_grid_widget, display_emojis_by_category_fn, display_arbitrary_emojis_fn) =
        create_emoji_grid_section(
            side_margin,
            vertical_margin,
            selected_category.clone(),
            all_emojis_by_category.clone(),
            window_ref.clone(),
            chosen_emoji.clone(),
        );

    let search_service = get_search_service(display_arbitrary_emojis_fn.clone());

    // Crear navegación de categorías
    let (category_nav, toggle_nav_class) = create_category_nav(
        side_margin,
        vertical_margin,
        selected_category.clone(),
        display_emojis_by_category_fn.clone(),
        search_service.cancel_pending_search_fn.clone(),
    );

    // Crear sección de búsqueda
    let search_section = create_search_section(
        side_margin,
        display_emojis_by_category_fn.clone(),
        selected_category.clone(),
        toggle_nav_class.clone(),
        search_service.initiate_debounced_search_fn.clone(),
        search_input_rc.clone(),
    );

    // Poniendo componentes en la caja principal
    main_box.append(&search_section);
    main_box.append(&category_nav);
    main_box.append(&emoji_grid_widget);

    let key_controller = EventControllerKey::new();
    let window_clone = window.clone();

    key_controller.connect_key_pressed(move |_controller, key, _keycode, _state| {
        if key == Key::Escape {
            window_clone.close();
            gtk::glib::Propagation::Stop
        } else {
            gtk::glib::Propagation::Proceed
        }
    });

    window.add_controller(key_controller);

    window.present();
}
