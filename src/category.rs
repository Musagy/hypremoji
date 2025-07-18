#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Category {
    Recents,
    SmileysAndEmotion,
    AnimalsAndNature,
    FoodAndDrink,
    Activities,
    TravelAndPlaces,
    Objects,
    Symbols,
    Flags,
}

impl Category {
    pub fn icon(&self) -> &str {
        match self {
            Category::Recents => "⏳",
            Category::SmileysAndEmotion => "😀",
            Category::AnimalsAndNature => "🐶",
            Category::FoodAndDrink => "🍎",
            Category::Activities => "⛹️",
            Category::TravelAndPlaces => "🌍",
            Category::Objects => "📖",
            Category::Symbols => "♾️",
            Category::Flags => "🚩",
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Category::Recents => "Recents",
            Category::SmileysAndEmotion => "Smileys & Emotion",
            Category::AnimalsAndNature => "Animals & Nature",
            Category::FoodAndDrink => "Food & Drink",
            Category::Activities => "Activities",
            Category::TravelAndPlaces => "Travel & Places",
            Category::Objects => "Objects",
            Category::Symbols => "Symbols",
            Category::Flags => "Flags",
        }
    }
}

// Emoji JSON structure

// {
//   "@version": "16.0.0",
//   "@author": "Chalda Pnuzig <chalda_emoji＠chalda.it>",
//   "@copyright": "Chalda Pnuzig 2021-2024",
//   "@see": "{@link https://github.com/chalda-pnuzig/emojis.json|GitHub}",
//   "@license": "ISC",
//   "emojis": {
//     "Smileys & Emotion": {
//       "subclass": [
//         {
//           "emoji": "😀",
//           "name": "grinning face",
//           "code": ["1F600"]
//         }
//         //...
//       ]
//     },
//     "People & Body": {
//       //...
//     },
//     "Component": {
//       //...
//     },
//     "Animals & Nature": {
//       //...
//     },
//     "Food & Drink": {
//       //...
//     },
//     "Travel & Places": {
//       //...
//     },
//     "Activities": {
//       //...
//     },
//     "Objects": {
//       //...
//     },
//     "Symbols": {
//       //...
//     },
//     "Flags": {
//       //...
//     }
//   }
// }
