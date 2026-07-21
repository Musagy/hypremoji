pub mod clipboard_manager;
pub mod config_writer;
pub mod emoji_loader;
pub mod get_client;
pub mod local_storage;
pub mod path_utils;
pub mod reset_config;
pub mod load_styles;
pub mod pin_utils;
pub mod setup;
pub mod fs_ops;

pub use clipboard_manager::get_clipboard_manager;
pub use config_writer::write_both_configs;
pub use emoji_loader::{
    find_emoji_by_name, load_all_emojis, load_emoji_for_category, EmojisListJsonRoot,
};
pub use get_client::{get_current_offset, get_hypremoji_client, get_last_client};
pub use local_storage::{add_emoji_to_recents, load_recents};
pub use path_utils::{get_assets_base_path, get_config_dir, get_base_path};
pub use reset_config::{reset_config, reset_css};
pub use load_styles::load_css;
pub use pin_utils::change_pin_type;
pub use setup::setup_hyprland;