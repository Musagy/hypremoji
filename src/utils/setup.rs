use std::error::Error;
use std::path::Path;
use crate::utils::fs_ops::{append_if_missing, is_root};

const LUA_MARKER: &str = "require(\"hypremoji\")";
const LUA_SNIPPET: &str = "\n-- HyprEmoji config\npackage.path = package.path .. \";\" .. os.getenv(\"HOME\") .. \"/.config/hypremoji/?.lua\"\nrequire(\"hypremoji\")\n";

const CONF_MARKER: &str = "hypremoji/hypremoji.conf";
const CONF_SNIPPET: &str = "\n# HyprEmoji config\nsource = ~/.config/hypremoji/hypremoji.conf\n";

pub fn setup_hyprland() -> Result<(), Box<dyn Error>> {
    if is_root() {
        return Err(Box::from(
            "Don't run this with sudo. Run `hypremoji setup-hyprland` as your \
             normal user so it edits your own ~/.config/hypr files, not root's.",
        ));
    }

    let hypr_dir = dirs::config_dir()
        .ok_or("Failed to determine config directory (~/.config)")?
        .join("hypr");

    let lua_path = hypr_dir.join("hyprland.lua");
    let conf_path = hypr_dir.join("hyprland.conf");

    if lua_path.exists() {
        return wire(&lua_path, LUA_MARKER, LUA_SNIPPET);
    }
    if conf_path.exists() {
        return wire(&conf_path, CONF_MARKER, CONF_SNIPPET);
    }

    Err(Box::from(format!(
        "No hyprland.lua or hyprland.conf found in '{}'. Set up Hyprland first.",
        hypr_dir.display()
    )))
}

fn wire(path: &Path, marker: &str, snippet: &str) -> Result<(), Box<dyn Error>> {
    match append_if_missing(path, marker, snippet)? {
        Some(backup_path) => {
            println!("-> Backup created: {}", backup_path.display());
            println!("-> Configuration added to {}", path.display());
            println!("-> Reload Hyprland to apply changes (SUPER + SHIFT + R)");
        }
        None => {
            println!("-> hypremoji is already wired into {}", path.display());
        }
    }
    Ok(())
}