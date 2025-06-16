use serde::{Deserialize, Serialize};
use std::{
    collections::VecDeque, // Usamos VecDeque para un control eficiente del "reciente" (añadir/quitar del frente/final)
    fs::File,
    io::{BufReader, BufWriter},
    path::PathBuf,
};

use crate::utils::get_config_dir;

const MAX_RECENTS: usize = 64;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct RecentsEmojis {
    pub emojis: VecDeque<String>,
}

fn get_recents_file_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let config_dir_path = get_config_dir()?;

    let recents_path = config_dir_path.join("recents.json");

    if !recents_path.exists() {
        let file =
            File::create(&recents_path).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

        serde_json::to_writer_pretty(file, &RecentsEmojis::default())
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
    }

    return Ok(recents_path);
}

pub fn load_recents() -> Result<RecentsEmojis, Box<dyn std::error::Error>> {
    let file_path = get_recents_file_path()?;

    let file = File::open(&file_path).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
    let reader = BufReader::new(file);

    let recents: RecentsEmojis =
        serde_json::from_reader(reader).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

    Ok(recents)
}

fn save_recents(recents: &RecentsEmojis) -> Result<(), Box<dyn std::error::Error>> {
    let file_path = get_recents_file_path()?;

    let file = File::create(&file_path).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

    let write = BufWriter::new(file);
    serde_json::to_writer_pretty(write, recents)
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

    println!(
        "Recientes guardados correctamente en: {}",
        file_path.display()
    );
    Ok(())
}

pub fn add_emoji_to_recents(emoji: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut recents = load_recents()?;

    recents.emojis.retain(|e| e != &emoji);
    recents.emojis.push_front(emoji);
    recents.emojis.truncate(MAX_RECENTS);

    save_recents(&recents)?;
    Ok(())
}
