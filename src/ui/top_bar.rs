use gtk::{
    glib::object::Cast,
    prelude::{BoxExt, EditableExt, EntryExt, EventControllerExt, WidgetExt},
    Box as BoxGtk, Entry, EventControllerFocus,
};
use std::{cell::RefCell, rc::Rc};

use crate::{
    category::Category,
    ui::{create_generic_btn, create_save_window_state_button, IconName},
};

pub fn create_top_bar(
    global_margin: i32,
    set_emojis_for_cat: Rc<RefCell<Box<dyn Fn(Category) + 'static>>>,
    selected_category: Rc<RefCell<Category>>,
    toggle_nav_class: Rc<dyn Fn(bool)>,
    initiate_debounced_search_fn: Rc<std::boxed::Box<dyn Fn(std::string::String)>>,
    search_input_global: Rc<RefCell<Entry>>,
) -> BoxGtk {
    let container = BoxGtk::new(gtk::Orientation::Horizontal, 8);
    // container.add_css_class("top-bar");
    container.set_margin_start(global_margin);
    container.set_margin_end(global_margin);

    let search_input = search_input_global.borrow().clone();
    search_input.set_hexpand(true); // Expande en horizontal
    search_input.set_placeholder_text(Some("Search emoji"));

    setup_search_events(
        search_input_global,
        set_emojis_for_cat,
        selected_category,
        toggle_nav_class,
        initiate_debounced_search_fn,
    );
    container.append(&search_input);

    let lock_win_btn = create_generic_btn(IconName::FaLockOpen, "lock-window-btn");
    lock_win_btn.set_tooltip_text(Some("Don't close the window\nwhen selecting an emoji"));
    container.append(&lock_win_btn);

    let save_window_state_btn = create_save_window_state_button();
    container.append(&save_window_state_btn);

    container
}

fn setup_search_events(
    search_input_rc: Rc<RefCell<Entry>>,
    set_emojis_for_cat: Rc<RefCell<Box<dyn Fn(Category) + 'static>>>,
    selected_category: Rc<RefCell<Category>>,
    toggle_nav_class: Rc<dyn Fn(bool)>,
    initiate_debounced_search_fn: Rc<std::boxed::Box<dyn Fn(std::string::String)>>,
) {
    let set_category_emojis_display_fn_clone = set_emojis_for_cat.clone();
    let selected_category_clone = selected_category.clone();
    let initiate_debounced_search_fn_clone = initiate_debounced_search_fn.clone();
    let search_input = search_input_rc.borrow().clone();

    // Cuando el Entry gana el foco
    let focus_controller = EventControllerFocus::new();
    focus_controller.connect_enter(move |controller| {
        let entry = controller
            .widget()
            .and_then(|w| w.downcast_ref::<Entry>().cloned());
        if let Some(entry) = entry {
            entry.add_css_class("focused"); // Agrega tu clase CSS
        }
    });
    focus_controller.connect_leave(move |controller| {
        let Some(entry) = controller
            .widget()
            .and_then(|w| w.downcast_ref::<Entry>().cloned())
        else {
            return;
        };
        entry.remove_css_class("focused");
    });
    search_input.add_controller(focus_controller);

    let search_input_clone = search_input.clone();
    search_input.connect_changed(move |entry| {
        let current_search_text = entry.text().to_string();

        if current_search_text.is_empty() {
            search_input_clone.remove_css_class("active");
            toggle_nav_class(true); // Activa la navegación
        } else {
            search_input_clone.add_css_class("active");
            toggle_nav_class(false); // Desactiva la navegación
        }

        if current_search_text.is_empty() {
            set_category_emojis_display_fn_clone.borrow()(selected_category_clone.borrow().clone());
        } else {
            initiate_debounced_search_fn_clone(current_search_text);
        }
    });
}
