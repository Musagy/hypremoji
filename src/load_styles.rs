use std::fs;

use gtk::{gdk::Display, CssProvider};

use crate::utils::{get_config_dir, reset_css};

pub fn load_css() -> Result<(), Box<dyn std::error::Error>> {
    let provider = CssProvider::new();

    let config_dir_path = get_config_dir()?;

    let user_main_css_path = config_dir_path.join("style.css");

    if !user_main_css_path.exists() {
        reset_css(&config_dir_path)?;
        println!("Created default CSS at: {}", user_main_css_path.display());
    }

    provider.load_from_string(&fs::read_to_string(&user_main_css_path).unwrap_or_else(|_| {
        eprintln!(
            "Failed to read main style file from '{}'. Using built-in default CSS instead.",
            user_main_css_path.display()
        );
        String::from(
            "/* Default window body and text color */ \
             window { background-color: #282A36; color: #F8F8F2; font-family: Inter, sans-serif; }",
        )
    }));

    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    Ok(())
}
