use std::{collections::HashMap, fs};

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
struct EmojisListJsonRoot {
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

// TODO: agregar un limite de rebusques para no hacer un bucle infinito
// (Caso mas común: un pendejo que spamme un string largo y no exista un emoji con ese nombre, bucle rebuscando si encuentra algo con menos caracteres)
pub fn find_emoji_by_name(name: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    println!("Searching for emoji by name: {}", name);

    let assets_base_path = get_assets_base_path()?;
    let json_path = assets_base_path.join("list.min.json");
    let raw = fs::read_to_string(&json_path).map_err(|e| Box::new(e))?;

    let root: EmojisListJsonRoot = serde_json::from_str(&raw).map_err(|e| Box::new(e))?;

    let mut found = IndexSet::new();
    let mut query = name.to_lowercase();

    while found.len() < MIN_SEARCH_LENGTH_RETURN && !query.is_empty() {
        for emoji in &root.emojis {
            let emoji_name = emoji.name.to_lowercase();
            if emoji_name.contains(&query) {
                found.insert(emoji.emoji.clone());
                if found.len() >= MIN_SEARCH_LENGTH_RETURN {
                    break;
                }
            }
        }
        query.pop(); // acorta la búsqueda si aún no hay suficientes resultados
    }

    let result = found.into_iter().collect::<Vec<_>>();
    println!("lista de emojis encontrados: {:?}", result);

    Ok(result)
}

fn append_matching_emojis(
    all_emojis: &[EmojiDetail],
    found_emojis: &mut Vec<EmojiDetail>,
    name: &str,
) {
    for emoji_detail in all_emojis.iter() {
        let emoji_name = emoji_detail.name.to_lowercase();
        let is_contains = emoji_name.contains(&name.to_lowercase());

        if is_contains {
            found_emojis.push(emoji_detail.clone());
        }
    }
}
