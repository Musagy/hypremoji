use gtk::prelude::{ButtonExt, WidgetExt};

use crate::utils::get_assets_base_path;

pub enum IconName {
    AiFillPushpin,
}

impl IconName {
    pub fn as_str(&self) -> &'static str {
        match self {
            IconName::AiFillPushpin => "AiFillPushpin.svg",
        }
    }
}

pub fn create_generic_btn(icon_name: IconName, css_class: &str) -> gtk::Button {
    let btn = gtk::Button::new();
    let icon_path = get_icon_path(icon_name);

    let icon = gtk::Image::from_file(icon_path);
    icon.set_pixel_size(24);

    btn.set_child(Some(&icon));
    btn.add_css_class(css_class);
    btn.add_css_class("generic-btn");

    btn
}

pub fn get_icon_path(icon_name: IconName) -> String {
    let assets_path = get_assets_base_path().expect("Failed to get assets base path");
    let icon_path = assets_path.join("icons").join(icon_name.as_str());
    icon_path.to_str().expect("Invalid icon path").to_string()
}
