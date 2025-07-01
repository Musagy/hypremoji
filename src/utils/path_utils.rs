use std::{env, path::PathBuf};

pub fn get_assets_base_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let mut current_exe_path = env::current_exe()?;
    current_exe_path.pop();

    let project_root = current_exe_path
        .parent()
        .ok_or("Could not find parent directory of the executable (target/)")?
        .parent()
        .ok_or("Could not find parent directory of the executable (project root)")?;

    let assets_path = project_root.join("assets");

    if !assets_path.exists() {
        return Err(Box::from("FOLDER NOT FOUND: assets folder does not exist"));
    }

    Ok(assets_path)
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
