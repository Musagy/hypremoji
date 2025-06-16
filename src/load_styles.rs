use std::fs;

use gtk::{gdk::Display, CssProvider};

use crate::utils::{get_assets_base_path, get_config_dir};

pub fn load_css() -> Result<(), Box<dyn std::error::Error>> {
    let provider = CssProvider::new();

    let config_dir_path = get_config_dir()?;

    let user_main_css_path = config_dir_path.join("style.css");

    if !user_main_css_path.exists() {
        let assets_base_path = get_assets_base_path()?;
        let default_app_css_path = assets_base_path.join("style.css");

        if default_app_css_path.exists() {
            fs::copy(&default_app_css_path, &user_main_css_path)
                .map_err(|e| {
                    eprintln!(
                        "Error al copiar el archivo de estilo por defecto de '{}' a '{}': {}",
                        default_app_css_path.display(),
                        user_main_css_path.display(),
                        e
                    );
                })
                .ok();
        } else {
            return Err(Box::from(format!(
                "FILE NOT FOUND: Don't exist default style in: '{}'",
                default_app_css_path.display()
            )));
        }
    }

    provider.load_from_string(&fs::read_to_string(&user_main_css_path).unwrap_or_else(|_| {
        eprintln!("Error al leer el archivo de estilo principal desde '{}'. Usando CSS por defecto en memoria.", user_main_css_path.display());
        String::from("/* Cuerpo de la ventana y color de texto por defecto */ window { background-color: #282A36; color: #F8F8F2; font-family: Inter, sans-serif; }")
    }));

    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    Ok(())
}

// guardado por si necesitamos usar el css generico de la carpeta assets
// pub fn load_css() {
//     let provider = CssProvider::new();
//     provider.load_from_string(include_str!("style.css"));

//     gtk::style_context_add_provider_for_display(
//         &Display::default().expect("Could not connect to a display."),
//         &provider,
//         gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
//     );
// }
