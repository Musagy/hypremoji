use std::{cell::RefCell, rc::Rc, time::Duration};

use gtk::glib::{timeout_add_local, ControlFlow, SourceId};

use crate::utils::{find_emoji_by_name, load_all_emojis, EmojisListJsonRoot};

pub struct SearchService {
    pub cancel_pending_search_fn: Rc<Box<dyn Fn() + 'static>>,
    pub initiate_debounced_search_fn: Rc<Box<dyn Fn(String) + 'static>>,
}

pub fn get_search_service(
    set_custom_emojis_display_fn: Rc<RefCell<Box<dyn Fn(Vec<String>) + 'static>>>,
) -> SearchService {
    let search_timeout_id: Rc<RefCell<Option<SourceId>>> = Rc::new(RefCell::new(None));
    let timeout_id_for_cancel = search_timeout_id.clone();

    let cancel_pending_search_fn: Rc<Box<dyn Fn() + 'static>> = Rc::new(Box::new(move || {
        if let Some(id) = timeout_id_for_cancel.borrow_mut().take() {
            id.remove();
        }
    }));

    let all_emoji_list = Rc::new(RefCell::new(match load_all_emojis() {
        Ok(emojis) => emojis,
        Err(e) => {
            eprintln!("Error loading emojis: {}", e);
            EmojisListJsonRoot { emojis: Vec::new() }
        }
    }));

    let initiate_debounced_search_fn: Rc<Box<dyn Fn(String) + 'static>> = Rc::new(Box::new({
        let timeout_id = search_timeout_id.clone();
        let all_emoji_list = all_emoji_list.clone();
        let set_display_fn = set_custom_emojis_display_fn.clone();

        move |search_text: String| {
            if let Some(id) = timeout_id.borrow_mut().take() {
                id.remove();
            }

            let current_text = search_text.clone();
            let display_fn = set_display_fn.clone();
            let timeout_ref = timeout_id.clone();
            let emoji_list_ref = all_emoji_list.clone();

            let id = timeout_add_local(Duration::from_millis(300), move || {
                match find_emoji_by_name(&current_text, &*emoji_list_ref.borrow()) {
                    Ok(results) => display_fn.borrow()(results),
                    Err(e) => {
                        eprintln!("Error searching emojis: {}", e);
                        display_fn.borrow()(Vec::new());
                    }
                }

                *timeout_ref.borrow_mut() = None;
                ControlFlow::Break
            });

            *timeout_id.borrow_mut() = Some(id);
        }
    }));

    SearchService {
        cancel_pending_search_fn,
        initiate_debounced_search_fn,
    }
}
