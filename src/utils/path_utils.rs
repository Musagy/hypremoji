use std::{env, path::PathBuf};

pub fn get_base_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let exe_path = env::current_exe()?;

    if exe_path.starts_with("/usr") {
        let system_path = PathBuf::from("/usr/share/hypremoji");
        if system_path.exists() {
            return Ok(system_path);
        } else {
            return Err("Installed in /usr, but '/usr/share/hypremoji' was not found.".into());
        }
    }

    let mut current_path = exe_path.clone();
    for _ in 0..5 {
        current_path = current_path.parent().unwrap_or(&current_path).to_path_buf();
        if current_path.join("assets").exists() {
            return Ok(current_path);
        }
    }

    Err("Could not locate the Hypremoji project root in any expected path.".into())
}

pub fn get_assets_base_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
    Ok(get_base_path()?.join("assets"))
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
