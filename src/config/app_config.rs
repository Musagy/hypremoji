use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};
use crate::utils::path_utils::get_config_dir;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PinMode {
    Point,
    CursorDown,
    CursorUp,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub shift_paste_apps: Vec<String>,
    pub current_pin_mode: PinMode,
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            shift_paste_apps: vec![
                "kitty".to_string(), "alacritty".to_string(), "foot".to_string(),
                "wezterm".to_string(), "terminator".to_string(), "tilix".to_string(),
                "gnome-terminal".to_string(), "konsole".to_string(), "xterm".to_string(),
                "urxvt".to_string(), "st".to_string(), "tmux".to_string(), "rio".to_string(),
            ],
            current_pin_mode: PinMode::CursorDown
        }
    }
}

impl AppConfig {
    pub fn needs_shift_for_paste(&self, window_class: &str) -> bool {
        let class_lower = window_class.to_lowercase();
        self.shift_paste_apps
            .iter()
            .any(|app| class_lower.contains(&app.to_lowercase()))
    }
    
    pub fn save_new_pin_config(&self, new_mode: PinMode) -> Result<(), Box<dyn std::error::Error>> {
        let mut updated = self.clone();
        updated.current_pin_mode = new_mode;

        let config_path = get_config_path()?;
        let config_json = serde_json::to_string_pretty(&updated)?;
        fs::write(&config_path, config_json)?;
        println!("New pin mode saved to config file successfully.");
        Ok(())
    }
}

fn get_config_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let config_dir = get_config_dir()?;
    Ok(config_dir.join("config.json"))
}

pub fn load_config() -> AppConfig {
    let config_path = match get_config_path() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Failed to get config path: {}. Using defaults.", e);
            return AppConfig::default();
        }
    };

    if !config_path.exists() {
        println!("config.json not found. Using defaults...");
        return AppConfig::default();
    }

    match fs::read_to_string(&config_path) {
        Ok(content) => match serde_json::from_str::<AppConfig>(&content) {
            Ok(config) => {
                println!("Configuration loaded from: {:?}", config_path);
                config
            }
            Err(e) => {
                eprintln!("Failed to parse config.json: {}. Using defaults.", e);
                AppConfig::default()
            }
        },
        Err(e) => {
            eprintln!("Failed to read config.json: {}. Using defaults.", e);
            AppConfig::default()
        }
    }
}
