use std::{env, path::PathBuf};

/// Obtiene la ruta base de la carpeta 'assets' relativa al ejecutable.
///
/// Asume que el ejecutable está en `target/release/hypremoji` (o similar)
/// y que la carpeta 'assets' está en la raíz del proyecto.
/// Por ejemplo: `tu_proyecto/assets/`
pub fn get_assets_base_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let mut current_exe_path = env::current_exe()?; // Obtiene la ruta del ejecutable actual
    current_exe_path.pop(); // Elimina el nombre del ejecutable para obtener el directorio padre

    // Subimos dos niveles desde el directorio del ejecutable
    // (ej. de `target/release/` a `target/`, y luego a la raíz del proyecto)
    let project_root = current_exe_path
        .parent() // Sube de `release/` a `target/`
        .ok_or("No se encontró el directorio padre del ejecutable (target/)")?
        .parent() // Sube de `target/` a la raíz del proyecto
        .ok_or("No se encontró el directorio padre del ejecutable (raíz del proyecto)")?;

    // Une la raíz del proyecto con la carpeta 'assets'
    let assets_path = project_root.join("assets");
    Ok(assets_path)
}
