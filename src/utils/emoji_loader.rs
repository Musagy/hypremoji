use std::{collections::HashMap, fs};

use serde::Deserialize;

use crate::category::Category;
use indexmap::IndexMap;

use super::{get_assets_base_path, load_recents};

#[derive(Debug, Deserialize, Clone)]
pub struct EmojiDetail {
    pub emoji: String,
    pub name: String,
}

// This struct maps the object containing dynamic subclasses
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
) -> Result<(HashMap<Category, Vec<String>>, Category), Box<dyn std::error::Error>> {
    let assets_base_path = get_assets_base_path()?;
    let json_path = assets_base_path.join("categories.min.json");
    let raw = fs::read_to_string(&json_path).map_err(|e| Box::new(e))?;

    let root: EmojisByCategoryJsonRoot = serde_json::from_str(&raw).map_err(|e| Box::new(e))?;

    let mut categorized_emojis: HashMap<Category, Vec<String>> = HashMap::new();

    let recent_data = load_recents()?;
    categorized_emojis.insert(Category::Recents, recent_data.emojis.into());

    // Iterate over the top-level categories from the JSON
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
                        "Warning: Unknown or unmapped JSON category: {}",
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

    // If Recents has at least one emoji, return that category. Otherwise, return the first available category with emojis
    let initial_category = if categorized_emojis
        .get(&Category::Recents)
        .map_or(false, |v| !v.is_empty())
    {
        Category::Recents
    } else {
        categorized_emojis
            .keys()
            .next()
            .cloned()
            .unwrap_or(Category::SmileysAndEmotion)
    };
    Ok((categorized_emojis, initial_category))
}

const MIN_SEARCH_LENGTH_RETURN: usize = 20;
const MAX_SEARCH_ITERATIONS: usize = 1;

pub fn find_emoji_by_name(
    name: &str,
    all_emojis_in_memory: &EmojisListJsonRoot,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let emoji_list = all_emojis_in_memory.emojis.clone();

    let mut found: Vec<String> = Vec::new();
    let mut iterations = 0;
    let mut query = name.to_lowercase();
    let mut remaining_emojis = emoji_list;

    while found.len() < MIN_SEARCH_LENGTH_RETURN && iterations < MAX_SEARCH_ITERATIONS {
        remaining_emojis.retain(|emoji| {
            if emoji.name.to_lowercase().contains(&query) {
                found.push(emoji.emoji.clone());
                false // remove from list for the next round
            } else {
                true
            }
        });

        query.pop(); // shorten the query if not enough results
        iterations += 1; // increment iteration counter
    }

    Ok(found)
}

pub fn load_all_emojis() -> Result<EmojisListJsonRoot, Box<dyn std::error::Error>> {
    let assets_base_path = get_assets_base_path()?;
    let json_path = assets_base_path.join("list.min.json");
    let raw = fs::read_to_string(&json_path).map_err(|e| Box::new(e))?;

    let root: EmojisListJsonRoot = serde_json::from_str(&raw).map_err(|e| Box::new(e))?;
    Ok(root)
}
