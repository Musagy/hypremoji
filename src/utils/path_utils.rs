use std::{env, path::PathBuf};

pub fn get_assets_base_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let exe_path = env::current_exe()?;

    // If installed system-wide (e.g. from /usr/bin), use system assets path
    if exe_path.starts_with("/usr") {
        let system_path = PathBuf::from("/usr/share/hypremoji/assets");
        if system_path.exists() {
            return Ok(system_path);
        } else {
            return Err(
                "Installed in /usr, but '/usr/share/hypremoji/assets' was not found.".into(),
            );
        }
    }

    // Development mode (e.g. cargo run), look for a nearby assets/ folder
    let mut current_path = exe_path.clone();
    for _ in 0..5 {
        current_path = current_path.parent().unwrap_or(&current_path).to_path_buf();
        let try_assets = current_path.join("assets");
        if try_assets.exists() {
            return Ok(try_assets);
        }
    }

    Err("Could not locate the 'assets' folder in any expected path.".into())
}

pub fn get_config_dir() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let mut config_dir = dirs::config_dir().ok_or("Could not get config directory")?;

    config_dir.push("hypremoji");

    create_file_if_not_exists(&config_dir)?;

    Ok(config_dir)
}

fn create_file_if_not_exists(file_path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    if let Err(e) = std::fs::create_dir_all(&file_path) {
        eprintln!(
            "Error creating config directory '{}': {}",
            file_path.display(),
            e
        );
        return Err(Box::new(e));
    };
    Ok(())
}
