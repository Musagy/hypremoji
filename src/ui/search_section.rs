use gtk::{
    prelude::{BoxExt, EntryExt, WidgetExt},
    Box, Button, Entry,
};

pub fn create_search_section(global_margin: i32) -> Box {
    let search_box = Box::new(gtk::Orientation::Horizontal, 0);
    search_box.set_margin_start(global_margin); // Margen izquierdo
    search_box.set_margin_end(global_margin); // Margen derecho

    let search_input = Entry::new();
    search_input.set_hexpand(true); // Expande en horizontal
    search_input.set_placeholder_text(Some("Search emoji"));

    let search_button = Button::with_label("Search");
    search_button.add_css_class("btn_search");

    search_box.append(&search_input);
    search_box.append(&search_button);

    search_box
}
