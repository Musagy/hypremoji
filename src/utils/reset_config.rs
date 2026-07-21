use std::{fs, path::Path};
use crate::utils::{get_assets_base_path, get_base_path, get_config_dir};
use crate::utils::fs_ops::{backup_dir, copy_default_file};

pub fn reset_config() -> Result<(), Box<dyn std::error::Error>> {
    let config_dir_path = get_config_dir()?;

    if config_dir_path.exists() {
        let backup_path = backup_dir(&config_dir_path)?;
        println!("Backed up old config to: {}", backup_path.display());
    }

    fs::create_dir_all(&config_dir_path)?;
    println!("Created fresh config directory at: {}", config_dir_path.display());

    reset_css(&config_dir_path)?;
    println!("Reset CSS to default.");

    reset_hypremoji_rule_for_hyprland(&config_dir_path)?;
    println!("Reset Hyprland rule to default.");

    copy_default_file(
        &get_base_path()?.join("config.json"),
        &config_dir_path.join("config.json"),
        "app config",
    )?;
    println!("Reset app config to default.");

    println!("Hypremoji configuration has been reset to default.");
    println!(
        "Note: this does NOT touch your hyprland.lua/hyprland.conf. \
         Run `hypremoji setup-hyprland` if you need to (re)wire it in."
    );
    Ok(())
}

pub fn reset_css(config_dir_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let default_css_path = get_assets_base_path()?.join("style.css");
    let css_path = config_dir_path.join("style.css");
    copy_default_file(&default_css_path, &css_path, "style file")
}

pub fn reset_hypremoji_rule_for_hyprland(
    config_dir_path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let base = get_base_path()?;

    copy_default_file(
        &base.join("hypremoji.conf"),
        &config_dir_path.join("hypremoji.conf"),
        "Hyprland rule (.conf)",
    )?;

    copy_default_file(
        &base.join("hypremoji.lua"),
        &config_dir_path.join("hypremoji.lua"),
        "Hyprland rule (.lua)",
    )?;

    Ok(())
}