use std::{collections::HashMap, fs, time::Instant};

use serde::Deserialize;

use crate::category::Category;
use indexmap::IndexMap;

use super::{get_assets_base_path, load_recents};

#[derive(Debug, Deserialize, Clone)]
pub struct EmojiDetail {
    pub emoji: String,
    pub name: String,
    // pub code: Vec<String>,
}

// Esta struct mapea el objeto que contiene las subclases dinámicas
#[derive(Debug, Deserialize)]
struct CategoryWrapper {
    #[serde(flatten)]
    pub subcategory: IndexMap<String, Vec<EmojiDetail>>,
}

#[derive(Debug, Deserialize)]
struct EmojisByCategoryJsonRoot {
    pub emojis: IndexMap<String, CategoryWrapper>,
}

#[derive(Debug, Deserialize)]
pub struct EmojisListJsonRoot {
    pub emojis: Vec<EmojiDetail>,
}

pub fn load_emoji_for_category(
) -> Result<HashMap<Category, Vec<String>>, Box<dyn std::error::Error>> {
    let assets_base_path = get_assets_base_path()?;
    let json_path = assets_base_path.join("categories.min.json");
    let raw = fs::read_to_string(&json_path).map_err(|e| Box::new(e))?;

    let root: EmojisByCategoryJsonRoot = serde_json::from_str(&raw).map_err(|e| Box::new(e))?;
    // println!("Emojis root loaded successfully: {:?}", root);

    let mut categorized_emojis: HashMap<Category, Vec<String>> = HashMap::new();

    // Inicializar la categoría Recientes vacía
    let recent_data = load_recents()?;
    categorized_emojis.insert(Category::Recents, recent_data.emojis.into());

    // Iterar sobre las categorías principales del JSON
    for (json_category_name, dynamic_subclasses_wrapper) in root.emojis {
        for (_subclass_name, emoji_details_vec) in dynamic_subclasses_wrapper.subcategory {
            let target_category = match json_category_name.as_str() {
                "Smileys & Emotion" | "People & Body" => Some(Category::SmileysAndEmotion),
                "Component" => None,
                "Animals & Nature" => Some(Category::AnimalsAndNature),
                "Food & Drink" => Some(Category::FoodAndDrink),
                "Travel & Places" => Some(Category::TravelAndPlaces),
                "Activities" => Some(Category::Activities),
                "Objects" => Some(Category::Objects),
                "Symbols" => Some(Category::Symbols),
                "Flags" => Some(Category::Flags),
                _ => {
                    eprintln!(
                        "Advertencia: Categoría JSON desconocida o no mapeada: {}",
                        json_category_name
                    );
                    None
                }
            };

            if let Some(category) = target_category {
                let emoji_list = categorized_emojis.entry(category).or_insert_with(Vec::new);
                for emoji_detail in emoji_details_vec {
                    emoji_list.push(emoji_detail.emoji);
                }
            }
        }
    }

    Ok(categorized_emojis)
}

const MIN_SEARCH_LENGTH_RETURN: usize = 20;
const MAX_SEARCH_ITERATIONS: usize = 1;

// TODO: actualizar y optimizarlo: ya que es una copia separada, eliminar los elementos del array para que no sea necesario hacer una búsqueda difusa
pub fn find_emoji_by_name(
    name: &str,
    all_emojis_in_memory: &EmojisListJsonRoot,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let start_time = Instant::now(); // ⏱️ Empezar cronómetro

    let emoji_list = all_emojis_in_memory.emojis.clone();

    let mut found: Vec<String> = Vec::new();
    let mut iterations = 0; // Para limitar la búsqueda difusa
    let mut query = name.to_lowercase();
    let mut remaining_emojis = emoji_list;

    while found.len() < MIN_SEARCH_LENGTH_RETURN && iterations < MAX_SEARCH_ITERATIONS {
        // for emoji in &emoji_list {
        //     // let emoji_name = emoji.name.to_lowercase();
        //     if emoji.name.to_lowercase().contains(&query) {
        //         found.insert(emoji.emoji.clone());
        //     }
        // }

        remaining_emojis.retain(|emoji| {
            if emoji.name.to_lowercase().contains(&query) {
                found.push(emoji.emoji.clone());
                false // elimina de la lista para la siguiente ronda
            } else {
                true
            }
        });

        query.pop(); // acorta la búsqueda si aún no hay suficientes resultados
        iterations += 1; // Incrementa el contador de iteraciones
    }

    let duration = start_time.elapsed(); // ⏱️ Calcular tiempo transcurrido
    println!(
        "🔍 Search for '{}' took: {:.2}ms (found {} emojis)",
        name,
        duration.as_secs_f64() * 1000.0,
        found.len()
    );

    Ok(found)
}

pub fn load_all_emojis() -> Result<EmojisListJsonRoot, Box<dyn std::error::Error>> {
    let assets_base_path = get_assets_base_path()?;
    let json_path = assets_base_path.join("list.min.json");
    let raw = fs::read_to_string(&json_path).map_err(|e| Box::new(e))?;

    let root: EmojisListJsonRoot = serde_json::from_str(&raw).map_err(|e| Box::new(e))?;
    Ok(root)
}
