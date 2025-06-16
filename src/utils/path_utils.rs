use std::{env, path::PathBuf};

pub fn get_assets_base_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let mut current_exe_path = env::current_exe()?;
    current_exe_path.pop();

    let project_root = current_exe_path
        .parent()
        .ok_or("No se encontró el directorio padre del ejecutable (target/)")?
        .parent()
        .ok_or("No se encontró el directorio padre del ejecutable (raíz del proyecto)")?;

    // Une la raíz del proyecto con la carpeta 'assets'
    let assets_path = project_root.join("assets");

    if !assets_path.exists() {
        return Err(Box::from("FOLDER NOT FOUND: assets folder does not exist"));
    }

    Ok(assets_path)
}

pub fn get_config_dir() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let mut config_dir =
        dirs::config_dir().ok_or("No se pudo obtener el directorio de configuración")?;

    config_dir.push("hypremoji");

    create_file_if_not_exists(&config_dir)?;

    Ok(config_dir)
}

fn create_file_if_not_exists(file_path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    if let Err(e) = std::fs::create_dir_all(&file_path) {
        eprintln!(
            "Error al crear el directorio de configuración '{}': {}",
            file_path.display(),
            e
        );
        return Err(Box::new(e));
    };
    Result::Ok(())
}
