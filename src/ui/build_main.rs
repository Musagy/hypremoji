use gtk::{gdk::Key, prelude::WidgetExt, EventControllerKey};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

use gtk::{
    prelude::{BoxExt, GtkWindowExt},
    Application, ApplicationWindow, Box as BoxGtk, Entry,
};

use crate::{
    category::Category,
    services::get_search_service,
    ui::{create_category_nav, create_emoji_grid_section, create_top_bar},
    utils::{clipboard_manager::ClipboardManager, load_emoji_for_category},
};

pub fn build_ui(app: &Application, cb_manager: ClipboardManager) {
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

    let (all_emojis_by_category, first_cat) = match load_emoji_for_category() {
        Ok((map, first_cat)) => (Rc::new(RefCell::new(map)), first_cat),
        Err(e) => {
            eprintln!("Failed to load emojis: {}", e);
            (Rc::new(RefCell::new(HashMap::new())), Category::Recents)
        }
    };

    let selected_category = Rc::new(RefCell::new(first_cat));

    let (emoji_grid_widget, display_emojis_by_category_fn, display_arbitrary_emojis_fn) =
        create_emoji_grid_section(
            side_margin,
            vertical_margin,
            selected_category.clone(),
            all_emojis_by_category.clone(),
            window_ref.clone(),
            cb_manager.clone(),
        );

    let search_service = get_search_service(display_arbitrary_emojis_fn.clone());

    let (category_nav, toggle_nav_class) = create_category_nav(
        side_margin,
        vertical_margin,
        selected_category.clone(),
        display_emojis_by_category_fn.clone(),
        search_service.cancel_pending_search_fn.clone(),
    );

    let search_section = create_top_bar(
        side_margin,
        display_emojis_by_category_fn.clone(),
        selected_category.clone(),
        toggle_nav_class.clone(),
        search_service.initiate_debounced_search_fn.clone(),
        search_input_rc.clone(),
    );

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
