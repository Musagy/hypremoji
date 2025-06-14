use std::{collections::HashMap, fs};

use serde::Deserialize;

use crate::category::Category;
use indexmap::IndexMap;

use super::get_assets_base_path;

#[derive(Debug, Deserialize, Clone)]
pub struct EmojiDetail {
    pub emoji: String,
    pub name: String,
    pub code: Vec<String>,
}

// Esta struct mapea el objeto que contiene las subclases dinámicas
#[derive(Debug, Deserialize)]
struct DynamicSubclassesWrapper {
    #[serde(flatten)]
    pub subclasses: IndexMap<String, Vec<EmojiDetail>>,
}

#[derive(Debug, Deserialize)]
struct EmojisJsonRoot {
    pub emojis: IndexMap<String, DynamicSubclassesWrapper>,
}

// pub const JSON_PATH: &str = "assets/categories.min.json";

pub fn load_emoji_for_category(
) -> Result<HashMap<Category, Vec<String>>, Box<dyn std::error::Error>> {
    let assets_base_path = get_assets_base_path()?;
    let json_path = assets_base_path.join("categories.min.json");
    let raw = fs::read_to_string(&json_path).map_err(|e| Box::new(e))?;

    let root: EmojisJsonRoot = serde_json::from_str(&raw).map_err(|e| Box::new(e))?;
    // println!("Emojis root loaded successfully: {:?}", root);

    let mut categorized_emojis: HashMap<Category, Vec<String>> = HashMap::new();

    // Inicializar la categoría Recientes vacía
    categorized_emojis.insert(Category::Recientes, Vec::new());

    // Iterar sobre las categorías principales del JSON
    for (json_category_name, dynamic_subclasses_wrapper) in root.emojis {
        for (_subclass_name, emoji_details_vec) in dynamic_subclasses_wrapper.subclasses {
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

    // println!(
    //     "todos los emojis de Smileys & Emotion: {:?}",
    //     categorized_emojis.get(&Category::SmileysAndEmotion)
    // );

    // Retorna el HashMap realmente poblado
    Ok(categorized_emojis)
}
