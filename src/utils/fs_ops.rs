use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use chrono::Local;

/// Builds a sibling path for `path` with a `-backup-<timestamp>` suffix
/// inserted before the extension, e.g.
/// `hyprland.lua` -> `hyprland-backup-20260720-153000.lua`
/// `~/.config/hypremoji` (no extension) -> `hypremoji-backup-20260720-153000`
fn timestamped_backup_path(path: &Path) -> PathBuf {
    let timestamp = Local::now().format("%Y%m%d-%H%M%S");
    let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("backup");

    match path.extension().and_then(|e| e.to_str()) {
        Some(ext) => path.with_file_name(format!("{stem}-backup-{timestamp}.{ext}")),
        None => path.with_file_name(format!("{stem}-backup-{timestamp}")),
    }
}

/// Copies `path` to a timestamped sibling backup, leaving the original
/// in place. Use before mutating a single file in place (e.g. appending
/// lines to hyprland.lua).
pub fn backup_file(path: &Path) -> Result<PathBuf, Box<dyn Error>> {
    let backup_path = timestamped_backup_path(path);
    fs::copy(path, &backup_path).map_err(|e| {
        format!("Could not create backup at '{}': {}", backup_path.display(), e)
    })?;
    Ok(backup_path)
}

/// Moves `dir` aside to a timestamped sibling backup. Use before
/// recreating a directory from scratch (e.g. `hypremoji reset`).
pub fn backup_dir(dir: &Path) -> Result<PathBuf, Box<dyn Error>> {
    let backup_path = timestamped_backup_path(dir);
    fs::rename(dir, &backup_path).map_err(|e| {
        format!(
            "Could not back up '{}' to '{}': {}",
            dir.display(),
            backup_path.display(),
            e
        )
    })?;
    Ok(backup_path)
}

/// Copies a default/template asset into place, with a consistent
/// "not found" / "copy failed" error message. Used by `reset` to
/// restore CSS, hypremoji.conf/.lua, and config.json.
pub fn copy_default_file(
    default_path: &Path,
    target_path: &Path,
    label: &str,
) -> Result<(), Box<dyn Error>> {
    if !default_path.exists() {
        return Err(Box::from(format!(
            "FILE NOT FOUND: default {} not found at '{}'",
            label,
            default_path.display()
        )));
    }

    fs::copy(default_path, target_path).map_err(|e| {
        format!(
            "Failed to copy default {} from '{}' to '{}': {}",
            label,
            default_path.display(),
            target_path.display(),
            e
        )
    })?;

    Ok(())
}

/// Appends `snippet` to `path` unless it already contains `marker`.
/// Backs up `path` first if it's going to write. Returns:
/// - `Ok(Some(backup_path))` if it wrote (and where the backup landed)
/// - `Ok(None)` if `marker` was already present (no-op, no backup made)
///
/// Used by `setup_hyprland` to wire the require()/source line in
/// idempotently and safely.
pub fn append_if_missing(
    path: &Path,
    marker: &str,
    snippet: &str,
) -> Result<Option<PathBuf>, Box<dyn Error>> {
    let contents = fs::read_to_string(path)
        .map_err(|e| format!("Could not read '{}': {}", path.display(), e))?;

    if contents.contains(marker) {
        return Ok(None);
    }

    let backup_path = backup_file(path)?;

    fs::write(path, format!("{contents}{snippet}"))
        .map_err(|e| format!("Could not write '{}': {}", path.display(), e))?;

    Ok(Some(backup_path))
}

/// True if the current process is running as root (e.g. under sudo).
/// Used to refuse touching a user's dotfiles when `$HOME` would
/// actually resolve to root's home instead of the real user's.
pub fn is_root() -> bool {
    std::process::Command::new("id")
        .arg("-u")
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim() == "0")
        .unwrap_or(false)
}