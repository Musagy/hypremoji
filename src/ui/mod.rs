// Declara los submódulos de la UI y los hace públicos para que main.rs pueda importarlos.
pub mod build_main;
pub mod category_nav;
pub mod emoji_grid;
pub mod generic_btn;
pub mod save_window_state_btn;
pub mod top_bar;

// Re-exporta las funciones para que puedan ser importadas directamente desde `ui::*`
pub use build_main::build_ui;
pub use category_nav::create_category_nav;
pub use emoji_grid::create_emoji_grid_section;
pub use generic_btn::{create_generic_btn, IconName};
pub use save_window_state_btn::create_save_window_state_button;
pub use top_bar::create_top_bar; // Renombrado para consistencia
