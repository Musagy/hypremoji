use std::{cell::RefCell, rc::Rc};

use gtk::{
    glib::{timeout_add_seconds_local, SourceId},
    prelude::{BoxExt, EditableExt, EntryExt, WidgetExt},
    Box as BoxGtk, Entry,
};

use crate::{category::Category, utils::find_emoji_by_name};

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

    let search_timeout_id: Rc<RefCell<Option<SourceId>>> = Rc::new(RefCell::new(None));

    let set_category_emojis_display_fn_clone = set_emojis_for_cat.clone();

    search_input.connect_changed(move |entry| {
        let current_search_text = entry.text().to_string();
        println!("Search text changed: {}", current_search_text);

        // Cancelar el temporizador anterior si existe
        if let Some(id) = search_timeout_id.borrow_mut().take() {
            id.remove();
        }

        // Pone Recientes por defecto al no tener texto de búsqueda
        if current_search_text.is_empty() {
            set_category_emojis_display_fn_clone.borrow()(Category::Recents);
            return;
        }

        let search_timeout_id_for_timeout = search_timeout_id.clone();
        let set_emojis_for_vec_clone = set_emojis_for_vec.clone();

        let id = timeout_add_seconds_local(1, move || {
            println!("Executing debounced search for: '{}'", current_search_text);

            match find_emoji_by_name(&current_search_text) {
                Ok(found_emoji) => {
                    let mut emojis_vec = Vec::new();
                    for emoji_detail in found_emoji {
                        emojis_vec.push(emoji_detail.emoji);
                    }
                    set_emojis_for_vec_clone.borrow()(emojis_vec);
                }
                Err(e) => {
                    eprintln!("Error searching emojis: {}", e);
                    // Aquí podrías mostrar un mensaje de error al usuario si lo deseas
                }
            }

            *search_timeout_id_for_timeout.borrow_mut() = None;
            gtk::glib::ControlFlow::Break
        });

        *search_timeout_id.borrow_mut() = Some(id);
    });

    search_box
}
