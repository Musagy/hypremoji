use gtk::{
    prelude::{ButtonExt, Cast, GridExt, ListModelExt, WidgetExt},
    Button, FlowBox, Grid,
};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::category::Category;

use super::refresh_flowbox;

pub fn create_category_nav(
    side_margin: i32,
    vertical_margin: i32,
    selected_category: Rc<RefCell<Category>>,
    all_emojis_by_category: Rc<RefCell<HashMap<Category, Vec<String>>>>,
    emoji_flowbox_ref: Rc<RefCell<FlowBox>>,
) -> Grid {
    let category_nav = Grid::new();
    category_nav.add_css_class("category_nav");
    category_nav.set_column_spacing(5);
    category_nav.set_column_homogeneous(true); // Opcional, para que los elementos hijos se distribuyan uniformemente
    category_nav.set_margin_start(side_margin); // Margen izquierdo
    category_nav.set_margin_end(side_margin); // Margen derecho
    category_nav.set_margin_top(vertical_margin); // Margen superior

    // Lista de categorías
    let categories = vec![
        Category::Recientes,
        Category::SmileysAndEmotion,
        Category::AnimalsAndNature,
        Category::FoodAndDrink,
        Category::TravelAndPlaces,
        Category::Activities,
        Category::Objects,
        Category::Symbols,
        Category::Flags,
    ];

    // Crear los botones con el ícono y el nombre de la categoría
    for (index, cat) in categories.iter().enumerate() {
        let selected_category_clone = selected_category.clone();
        let current_cat_clone = cat.clone(); // Clonamos la categoría actual para el closure
        let category_nav_clone_for_button = category_nav.clone();
        let all_emojis_by_category_clone = all_emojis_by_category.clone();
        let emoji_flowbox_ref_clone = emoji_flowbox_ref.clone();

        let btn = Button::with_label(cat.icon());
        btn.set_tooltip_text(Some(cat.name()));
        btn.add_css_class("category-button"); // Aplica clase CSS

        if cat == &*selected_category.borrow() {
            btn.add_css_class("active");
        }

        btn.connect_clicked(move |_| {
            // Actualiza la categoría seleccionada
            *selected_category_clone.borrow_mut() = current_cat_clone.clone();
            println!("Categoría seleccionada: {}", current_cat_clone.name());

            // Itera sobre los hijos de la cuadrícula de categorías para actualizar las clases 'active'
            for i in 0..category_nav_clone_for_button.observe_children().n_items() {
                if let Some(child_obj) = category_nav_clone_for_button.observe_children().item(i) {
                    if let Ok(button) = child_obj.downcast::<Button>() {
                        if button.label().unwrap().as_str() == current_cat_clone.icon() {
                            button.add_css_class("active");
                        } else {
                            button.remove_css_class("active");
                        }
                    }
                }
            }

            let current_flowbox = emoji_flowbox_ref_clone.borrow();
            let all_emojis = all_emojis_by_category_clone.borrow();

            let emojis_to_show = all_emojis
                .get(&current_cat_clone)
                .map_or(Vec::new(), |vec| vec.clone());

            refresh_flowbox(&current_flowbox, emojis_to_show);
        });

        category_nav.attach(&btn, index as i32, 0, 1, 1);
    }
    category_nav
}
