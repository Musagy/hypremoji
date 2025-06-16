use std::{cell::RefCell, collections::HashMap, process::Command, rc::Rc};

use crate::{category::Category, utils::add_emoji_to_recents};
use crate::utils::path_utils::get_assets_base_path;
use gtk::{
    prelude::{BoxExt, Cast, FlowBoxChildExt, GtkWindowExt, WidgetExt},
    ApplicationWindow, Box, FlowBox, FlowBoxChild, Label, ScrolledWindow,
};

pub fn refresh_flowbox(flowbox: &FlowBox, emoji_list: Vec<String>) {
    // Limpieza
    while let Some(child) = flowbox.first_child() {
        flowbox.remove(&child);
    }

    // Rellena
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

pub fn create_emoji_grid_section(
    side_margin: i32,
    vertical_margin: i32,
    initial_category: Rc<RefCell<Category>>,
    all_emojis_by_category: Rc<RefCell<HashMap<Category, Vec<String>>>>,
    window_ref: Rc<RefCell<ApplicationWindow>>,
) -> (ScrolledWindow, Rc<RefCell<FlowBox>>) {
    let gap = 4;
    let emoji_flowbox = FlowBox::new();
    emoji_flowbox.set_selection_mode(gtk::SelectionMode::None); // No permite selección de múltiples emojis
    emoji_flowbox.set_row_spacing(gap / 2);
    emoji_flowbox.set_column_spacing(gap);
    emoji_flowbox.set_max_children_per_line(300); // Puedes ajustar este valor
    emoji_flowbox.set_min_children_per_line(5); // Puedes ajustar este valor, o dejarlo en 0 para que se ajuste más libremente
    emoji_flowbox.add_css_class("emoji_flowbox");

    emoji_flowbox.set_activate_on_single_click(true);

    let window_ref_clone = window_ref.clone();
    emoji_flowbox.connect_child_activated(move |_, flowbox_child| {
        if let Some(child_widget) = flowbox_child.child() {
            if let Ok(label) = child_widget.downcast::<Label>() {
                let emoji = label.text();
                println!("Emoji seleccionado: {}", emoji);

                let script_path_result = get_assets_base_path()
                    .map(|p| p.join("insert_emoji.sh"));

                match script_path_result {
                    Ok(script_path) => {
                        let command_str = format!("{} \"{}\"", script_path.display(), emoji);

                        add_emoji_to_recents(emoji.to_string())
                        .unwrap_or_else(|e| {
                            eprintln!("Error al añadir emoji a recientes: {}", e);
                        });

                        let result = Command::new("bash").arg("-c").arg(&command_str).spawn();

                        match result {
                            Ok(_) => {
                                println!("Comando bash iniciado en segundo plano: {}", command_str);
                            }
                            Err(e) => {
                                eprintln!("Fallo al intentar iniciar el comando bash: {}", e);
                                eprintln!("Asegúrate de que el script '{}' tenga permisos de ejecución: chmod +x {}", script_path.display(), script_path.display());
                            }
                        }
                    },
                    Err(e) => {
                        eprintln!("Error al obtener la ruta base de assets: {}", e);
                    }
                }
                
                window_ref_clone.borrow().close(); // Cierra la ventana al seleccionar un emoji
            }
        }
    });

    let initial_category_name = initial_category.borrow().clone();
    let emojis_map_borrow = all_emojis_by_category.borrow();

    let emojis_to_show = emojis_map_borrow
        .get(&initial_category_name)
        .map_or(Vec::new(), |vec| vec.clone());

    refresh_flowbox(&emoji_flowbox, emojis_to_show);

    let content_container = Box::new(gtk::Orientation::Vertical, 0); // Un Box vertical, sin espaciado interno propio
    content_container.append(&emoji_flowbox);

    let margin = 6;
    content_container.set_margin_start(side_margin);
    content_container.set_margin_end(side_margin / 2); // Margen a la derecha con scroll
    content_container.set_margin_top(vertical_margin);
    content_container.set_margin_bottom(margin);

    let scrolled_window = ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never) // No queremos una barra de desplazamiento horizontal
        .vscrollbar_policy(gtk::PolicyType::Automatic) // Queremos una barra de desplazamiento vertical automática
        .child(&content_container) // El FlowBox es el contenido que queremos que se desplace
        .build();

    // HACER QUE EL SCROLLED_WINDOW SE EXPANDA EN VERTICAL
    scrolled_window.set_vexpand(true);
    scrolled_window.set_hexpand(true);
    scrolled_window.set_margin_end(side_margin / 2); // Margen derecho fuera del scroll

    (scrolled_window, Rc::new(RefCell::new(emoji_flowbox)))
}
