use gtk::{
    prelude::{ButtonExt, Cast, GridExt, ListModelExt, WidgetExt},
    Button, Grid,
};
use std::{cell::RefCell, rc::Rc};

use crate::category::Category;

pub fn create_category_nav(
    side_margin: i32,
    vertical_margin: i32,
    selected_category: Rc<RefCell<Category>>,
    display_emojis_by_category_fn: Rc<RefCell<Box<dyn Fn(Category) + 'static>>>,
    cancel_pending_search_fn: Rc<Box<dyn Fn() + 'static>>,
) -> (Grid, Rc<dyn Fn(bool)>) {
    let category_nav = Grid::new();
    category_nav.add_css_class("category_nav");
    category_nav.add_css_class("nav_enabled");
    category_nav.set_column_spacing(5);
    category_nav.set_column_homogeneous(true); // Opcional, para que los elementos hijos se distribuyan uniformemente
    category_nav.set_margin_start(side_margin); // Margen izquierdo
    category_nav.set_margin_end(side_margin); // Margen derecho
    category_nav.set_margin_top(vertical_margin); // Margen superior

    let category_nav_clone_for_toggle = category_nav.clone();
    let toggle_nav_class_rc: Rc<dyn Fn(bool)> = Rc::new(move |enable: bool| {
        if enable {
            category_nav_clone_for_toggle.add_css_class("nav_enabled");
        } else {
            category_nav_clone_for_toggle.remove_css_class("nav_enabled");
        }
    });

    // Lista de categorías
    let categories = vec![
        Category::Recents,
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

        let display_emojis_by_category_fn_clone = display_emojis_by_category_fn.clone();

        let toggle_nav_class_clone_for_button = toggle_nav_class_rc.clone();
        let cancel_pending_search_fn_clone = cancel_pending_search_fn.clone();

        let btn = Button::with_label(cat.icon());
        btn.set_tooltip_text(Some(cat.name()));
        btn.add_css_class("category-button"); // Aplica clase CSS

        if cat == &*selected_category.borrow() {
            btn.add_css_class("active");
        }

        btn.connect_clicked(move |_| {
            cancel_pending_search_fn_clone();
            // Actualiza la categoría seleccionada
            *selected_category_clone.borrow_mut() = current_cat_clone.clone();
            // Cancela cualquier búsqueda pendiente

            // Itera sobre los hijos de la cuadrícula de categorías para actualizar las clases 'active'
            for i in 0..category_nav_clone_for_button.observe_children().n_items() {
                if let Some(child_obj) = category_nav_clone_for_button.observe_children().item(i) {
                    if let Ok(button) = child_obj.downcast::<Button>() {
                        if button.label().unwrap().as_str() == current_cat_clone.icon() {
                            button.add_css_class("active");
                            toggle_nav_class_clone_for_button(true); // Activa la clase nav_enabled
                        } else {
                            button.remove_css_class("active");
                        }
                    }
                }
            }

            display_emojis_by_category_fn_clone.borrow()(current_cat_clone.clone());
        });

        category_nav.attach(&btn, index as i32, 0, 1, 1);
    }
    (category_nav, toggle_nav_class_rc)
}
