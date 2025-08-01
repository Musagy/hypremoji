use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    category::Category,
    utils::{add_emoji_to_recents, clipboard_manager::ClipboardManager},
};
use gtk::{
    prelude::{BoxExt, Cast, FlowBoxChildExt, GtkWindowExt, WidgetExt},
    ApplicationWindow, Box as BoxGtk, FlowBox, FlowBoxChild, Label, ScrolledWindow,
};

pub fn refresh_flowbox(flowbox: &FlowBox, emoji_list: Vec<String>) {
    while let Some(child) = flowbox.first_child() {
        flowbox.remove(&child);
    }

    for emoji in emoji_list.iter() {
        let label = Label::new(Some(emoji));
        label.add_css_class("emoji-label");
        label.set_halign(gtk::Align::Center);
        label.set_valign(gtk::Align::Center);

        let flow_child = FlowBoxChild::new();
        flow_child.set_can_focus(true);
        flow_child.add_css_class("emoji-item");
        flow_child.set_child(Some(&label));
        flowbox.append(&flow_child);
    }
}

fn internal_refresh_emoji_display(
    flowbox_rc: &Rc<RefCell<FlowBox>>,
    all_emojis_by_category: &Rc<RefCell<HashMap<Category, Vec<String>>>>,
    category_to_show: &Category,
) {
    let current_flowbox = flowbox_rc.borrow();
    let all_emojis = all_emojis_by_category.borrow();
    let emojis_to_show = all_emojis
        .get(category_to_show)
        .map_or(Vec::new(), |vec| vec.clone());

    refresh_flowbox(&current_flowbox, emojis_to_show);
}

fn internal_display_arbitrary_emojis(
    flowbox_rc: &Rc<RefCell<FlowBox>>,
    emojis_to_display: Vec<String>,
) {
    let current_flowbox = flowbox_rc.borrow();
    refresh_flowbox(&current_flowbox, emojis_to_display);
}

pub fn create_emoji_grid_section(
    side_margin: i32,
    vertical_margin: i32,
    initial_category: Rc<RefCell<Category>>,
    all_emojis_by_category: Rc<RefCell<HashMap<Category, Vec<String>>>>,
    window_ref: Rc<RefCell<ApplicationWindow>>,
    cb_manager: ClipboardManager,
) -> (
    ScrolledWindow,
    Rc<RefCell<Box<dyn Fn(Category) + 'static>>>,
    Rc<RefCell<Box<dyn Fn(Vec<String>) + 'static>>>,
) {
    let gap = 4;
    let emoji_flowbox = FlowBox::new();
    emoji_flowbox.set_selection_mode(gtk::SelectionMode::None);
    emoji_flowbox.set_row_spacing(gap / 2);
    emoji_flowbox.set_column_spacing(gap);
    emoji_flowbox.set_max_children_per_line(300);
    emoji_flowbox.set_min_children_per_line(5);
    emoji_flowbox.add_css_class("emoji_flowbox");
    emoji_flowbox.set_activate_on_single_click(true);

    let emoji_flowbox_rc = Rc::new(RefCell::new(emoji_flowbox));
    let window_ref_clone = window_ref.clone();
    let cb_manager_clone = cb_manager.clone();

    emoji_flowbox_rc
        .borrow()
        .connect_child_activated(move |_, flowbox_child| {
            if let Some(child_widget) = flowbox_child.child() {
                if let Ok(label) = child_widget.downcast::<Label>() {
                    let emoji = label.text();

                    add_emoji_to_recents(emoji.to_string()).unwrap_or_else(|e| {
                        eprintln!("Failed to add emoji to recents: {}", e);
                    });

                    cb_manager_clone.set_chosen_emoji(emoji.to_string());
                    window_ref_clone.borrow().close();
                }
            }
        });

    let set_category_emojis_display = Rc::new(RefCell::new(Box::new({
        let emoji_flowbox_rc = emoji_flowbox_rc.clone();
        let all_emojis_by_category = all_emojis_by_category.clone();
        move |category: Category| {
            internal_refresh_emoji_display(&emoji_flowbox_rc, &all_emojis_by_category, &category);
        }
    })
        as Box<dyn Fn(Category) + 'static>));

    let set_custom_emojis_display = Rc::new(RefCell::new(Box::new({
        let emoji_flowbox_rc = emoji_flowbox_rc.clone();
        move |emojis: Vec<String>| {
            internal_display_arbitrary_emojis(&emoji_flowbox_rc, emojis);
        }
    })
        as Box<dyn Fn(Vec<String>) + 'static>));

    let initial_category_name = initial_category.borrow().clone();
    set_category_emojis_display.borrow()(initial_category_name);

    let content_container = BoxGtk::new(gtk::Orientation::Vertical, 0);
    content_container.append(&emoji_flowbox_rc.borrow().clone());

    let margin = 6;
    content_container.set_margin_start(side_margin);
    content_container.set_margin_end(side_margin / 2);
    content_container.set_margin_top(vertical_margin);
    content_container.set_margin_bottom(margin);

    let scrolled_window = ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never)
        .vscrollbar_policy(gtk::PolicyType::Automatic)
        .child(&content_container)
        .build();

    scrolled_window.set_vexpand(true);
    scrolled_window.set_hexpand(true);
    scrolled_window.set_margin_end(side_margin / 2);

    (
        scrolled_window,
        set_category_emojis_display,
        set_custom_emojis_display,
    )
}
