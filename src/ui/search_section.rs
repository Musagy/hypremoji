use gtk::{
    prelude::{BoxExt, EntryExt, WidgetExt},
    Box as BoxGtk, Entry,
};
use std::{cell::RefCell, rc::Rc};

use crate::category::Category;

pub fn create_search_section(
    global_margin: i32,
    set_emojis_for_vec: Rc<RefCell<Box<dyn Fn(Vec<String>) + 'static>>>,
    set_emojis_for_cat: Rc<RefCell<Box<dyn Fn(Category) + 'static>>>,
) -> BoxGtk {
    let search_box = BoxGtk::new(gtk::Orientation::Horizontal, 0);
    search_box.set_margin_start(global_margin); // Margen izquierdo
    search_box.set_margin_end(global_margin); // Margen derecho

    let search_input = Entry::new();
    search_input.set_hexpand(true); // Expande en horizontal
    search_input.set_placeholder_text(Some("Search emoji"));

    // let search_button = Button::with_label("Search");
    // search_button.add_css_class("btn_search");

    search_box.append(&search_input);
    // search_box.append(&search_button);

    search_box
}
