use std::{cell::RefCell, rc::Rc};

use gtk::{
    glib,
    prelude::{ButtonExt, WidgetExt},
    Button,
};

use crate::{
    config::app_config::{AppConfig, PinMode}, 
    ui::{IconName, create_generic_btn, generic_btn::update_btn_icon, toast::Toast}, 
    utils::change_pin_type
};

pub fn create_pin_btn(config: &AppConfig, toast: &Toast) -> gtk::Button {
    let initial_mode = config.current_pin_mode;
    
    let pin_btn = create_generic_btn(IconName::Resize, "pin-window-btn");
    pin_btn.set_tooltip_text(Some("Toggle pin mode\nFirst click: Refresh state and window size"));

    let current_mode = Rc::new(RefCell::new(initial_mode));
    // let is_first_click = Rc::new(RefCell::new(true));

    setup_pin_toggle(pin_btn.clone(), current_mode, config, toast);
    
    pin_btn
}

fn setup_pin_toggle(btn: Button, current_mode: Rc<RefCell<PinMode>>, config: &AppConfig, toast: &Toast) {
    let config_clone = config.clone();
    let toast_container = toast.container.clone();
    let toast_label = toast.label.clone();
    let first_click = Rc::new(RefCell::new(true));
    
    btn.connect_clicked(move |btn| {
        let current = *current_mode.borrow();
        
        let toast = Toast {
            container: toast_container.clone(),
            label: toast_label.clone(),
        };
        
        if let Err(e) = change_pin_type(current) {
            eprintln!("Error updating hyprland config: {}", e);
            toast.show_error(&format!("Error: {}", e));
            return;
        }
        
        if let Err(e) = config_clone.save_new_pin_config(current) {
            eprintln!("Error saving pin mode to config: {}", e);
            toast.show_error(&format!("Failed to save: {}", e));
            return;
        }
        
        println!("Saved pin mode: {:?}", current);
        if *first_click.borrow() {
            toast.show_message(&format!("✅ State refreshed"));
            *first_click.borrow_mut() = false;
        } else {
            toast.show_message(&format!("✅ Pin {:?} mode", current));
        }
        
        let next_mode = cycle_pin_mode(current);
        *current_mode.borrow_mut() = next_mode;
        
        update_btn_icon(btn, get_icon_for_mode(next_mode));
        trigger_button_animation(btn);
    });
}


fn get_icon_for_mode(mode: PinMode) -> IconName {
    match mode {
        PinMode::CursorUp => IconName::MouseDown,
        PinMode::CursorDown => IconName::MouseUp,
        PinMode::Point => IconName::AiFillPushpin,
    }
}

fn cycle_pin_mode(current: PinMode) -> PinMode {
    match current {
        PinMode::CursorUp => PinMode::CursorDown,
        PinMode::CursorDown => PinMode::Point,
        PinMode::Point => PinMode::CursorUp,
    }
}

fn trigger_button_animation(btn: &Button) {
    btn.remove_css_class("shot-animation");
    
    let btn_clone = btn.clone();
    glib::timeout_add_local_once(std::time::Duration::from_millis(10), move || {
        btn_clone.add_css_class("shot-animation");
        
        let btn_for_removal = btn_clone.clone();
        glib::timeout_add_local_once(std::time::Duration::from_millis(1000), move || {
            btn_for_removal.remove_css_class("shot-animation");
        });
    });
}