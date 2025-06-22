use std::cell::RefCell;
use std::rc::Rc;

use gtk::glib::timeout_add_seconds_local;
use gtk::glib::ControlFlow;
use gtk::glib::SourceId;

use crate::utils::find_emoji_by_name;

pub struct SearchService {
    pub cancel_pending_search_fn: Rc<Box<dyn Fn() + 'static>>,
    pub initiate_debounced_search_fn: Rc<Box<dyn Fn(String) + 'static>>,
}

pub fn set_search_service(
    set_custom_emojis_display_fn: Rc<RefCell<Box<dyn Fn(Vec<String>) + 'static>>>,
) -> SearchService {
    let search_timeout_id_global: Rc<RefCell<Option<SourceId>>> = Rc::new(RefCell::new(None));

    let search_timeout_id_for_cancel = search_timeout_id_global.clone();

    let cancel_pending_search_fn: Rc<Box<dyn Fn() + 'static>> = Rc::new(Box::new(move || {
        if let Some(id) = search_timeout_id_for_cancel.borrow_mut().take() {
            id.remove(); // Cancela el temporizador
            println!("Pending search cancelled.");
        }
    }));

    let initiate_debounced_search_fn: Rc<Box<dyn Fn(String) + 'static>> = Rc::new(Box::new({
        let search_timeout_id_global_clone_for_debounce = search_timeout_id_global.clone();
        let set_custom_emojis_display_fn_clone = set_custom_emojis_display_fn.clone();

        move |search_text: String| {
            // Cancelar el temporizador anterior si existe
            if let Some(id) = search_timeout_id_global_clone_for_debounce
                .borrow_mut()
                .take()
            {
                id.remove();
            }

            let current_search_text_clone = search_text.clone(); // Clonar para el closure del temporizador
            let set_emojis_for_vec_clone_for_timeout = set_custom_emojis_display_fn_clone.clone();
            let search_timeout_id_global_for_timeout =
                search_timeout_id_global_clone_for_debounce.clone();

            let id = timeout_add_seconds_local(1, move || {
                println!(
                    "Executing debounced search for: '{}'",
                    current_search_text_clone
                );

                match find_emoji_by_name(&current_search_text_clone) {
                    Ok(found_emoji_details) => {
                        set_emojis_for_vec_clone_for_timeout.borrow()(found_emoji_details);
                    }
                    Err(e) => {
                        eprintln!("Error searching emojis: {}", e);
                        set_emojis_for_vec_clone_for_timeout.borrow()(Vec::new());
                    }
                }
                *search_timeout_id_global_for_timeout.borrow_mut() = None;
                ControlFlow::Break
            });

            *search_timeout_id_global_clone_for_debounce.borrow_mut() = Some(id);
        }
    }));

    SearchService {
        cancel_pending_search_fn,
        initiate_debounced_search_fn,
    }
}
