// Declara los submódulos de la UI y los hace públicos para que main.rs pueda importarlos.
pub mod category_nav;
pub mod emoji_grid;
pub mod search_section;

// Re-exporta las funciones para que puedan ser importadas directamente desde `ui::*`
pub use category_nav::create_category_nav;
pub use emoji_grid::create_emoji_grid_section;
pub use emoji_grid::refresh_flowbox;
pub use search_section::create_search_section; // Renombrado para consistencia
