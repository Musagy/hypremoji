// Declara los submódulos de la UI y los hace públicos para que main.rs pueda importarlos.
pub mod build_main;
pub mod category_nav;
pub mod emoji_grid;
pub mod search_section;

// Re-exporta las funciones para que puedan ser importadas directamente desde `ui::*`
pub use build_main::build_ui;
pub use category_nav::create_category_nav;
pub use emoji_grid::create_emoji_grid_section;
pub use search_section::create_search_section; // Renombrado para consistencia
