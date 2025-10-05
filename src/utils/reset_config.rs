use std::{fs::{self, OpenOptions}, io::{self, BufRead, BufReader, Write}, path::Path};
use chrono::Local;
use crate::utils::{get_assets_base_path, get_base_path, get_config_dir};

pub fn reset_config() -> Result<(), Box<dyn std::error::Error>> {
    let config_dir_path = get_config_dir()?;

    if config_dir_path.exists() {
        let timestamp = Local::now().format("%Y%m%d-%H%M%S");
        let backup_path = config_dir_path
            .with_file_name(format!("hypremoji-backup-{}", timestamp));

        fs::rename(&config_dir_path, &backup_path)?;
        println!("Backed up old config to: {}", backup_path.display());
    }

    fs::create_dir_all(&config_dir_path)?;
    println!("Created fresh config directory at: {}", config_dir_path.display());
    
    reset_css(&config_dir_path)?;
    println!("Reset CSS to default.");

    reset_hypremoji_rule_for_hyprland(&config_dir_path)?;
    println!("Reset Hyprland rule to default.");

    ensure_hyprland_conf_includes_hypremoji()?;

    println!("Hypremoji configuration has been reset to default.");
    Ok(())
}

pub fn reset_css(config_dir_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let css_path = config_dir_path.join("style.css");
    let default_app_css_path = get_assets_base_path()?.join("style.css");

    if default_app_css_path.exists() {
        fs::copy(&default_app_css_path, &css_path).map_err(|e| {
            format!(
                "Failed to copy default style file from '{}' to '{}': {}",
                default_app_css_path.display(),
                css_path.display(),
                e
            )
        })?;
    } else {
        return Err(Box::from(format!(
            "FILE NOT FOUND: Don't exist default style in: '{}'",
            default_app_css_path.display()
        )));
    }
    Ok(())
}

pub fn reset_hypremoji_rule_for_hyprland(config_dir_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let rules_path = config_dir_path.join("hypremoji.conf");
    let default_rule_path = get_base_path()?.join("hypremoji.conf");
    
    if default_rule_path.exists() {
        fs::copy(&default_rule_path, &rules_path).map_err(|e| {
            format!(
                "Failed to copy default Hyprland rule file from '{}' to '{}': {}",
                default_rule_path.display(),
                rules_path.display(),
                e
            )
        })?;
    } else {
        return Err(Box::from(format!(
            "FILE NOT FOUND: Don't exist default Hyprland rule in: '{}'",
            default_rule_path.display()
        )));
    }
    Ok(())
}

pub fn ensure_hyprland_conf_includes_hypremoji() -> Result<(), Box<dyn std::error::Error>> {
    let hyprland_conf_path = dirs::config_dir()
        .map(|d| d.join("hypr/hyprland.conf"))
        .ok_or("Failed to determine config directory (~/.config)")?;

    if !hyprland_conf_path.exists() {
        return Err(Box::from(format!(
            "FILE NOT FOUND: Hyprland config not found at '{}'",
            hyprland_conf_path.display()
        )));
    }

    println!("Hypremoji will append the following lines to your hyprland.conf:\n");
    println!("# HyprEmoji config");
    println!("source = ~/.config/hypremoji/hypremoji.conf\n");
    print!("Do you want to continue? (Y/n): ");
    io::stdout().flush()?; 

    // Read user input
    let mut answer = String::new();
    io::stdin().read_line(&mut answer)?;
    let answer = answer.trim().to_lowercase(); 

    if !(answer == "y" || answer.is_empty()) {
        println!("Operation cancelled by user.");
        return Ok(());
    }

    // Read file content to check if it already includes Hypremoji config
    let file = fs::File::open(&hyprland_conf_path)?;
    let reader = BufReader::new(file);
    let content: Vec<String> = reader.lines().collect::<Result<_, _>>()?;

    let already_has_hypremoji = content.iter().any(|line| line.contains("hypremoji.conf"));
    if already_has_hypremoji {
        println!("The file already contains the Hypremoji config, nothing was added.");
        return Ok(());
    }

    // Append lines to the end of the file
    let mut file = OpenOptions::new().append(true).open(&hyprland_conf_path)?;
    writeln!(file, "\n# HyprEmoji config")?;
    writeln!(file, "source = ~/.config/hypremoji/hypremoji.conf")?;

    println!("Successfully added Hypremoji config to '{}'.", hyprland_conf_path.display());
    Ok(())
}
