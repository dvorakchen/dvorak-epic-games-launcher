use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameCover {
    pub id: String,
    pub name: String,
    pub cover_url: String,
    pub achievements_amount: usize,
    pub achievements_completed: usize,
}

impl Default for GameCover {
    fn default() -> Self {
        GameCover {
            id: "0".to_owned(),
            name: "Black Myth Wukong".to_owned(),
            cover_url: "black-myth-wukong.jpg".to_owned(),
            achievements_amount: 10,
            achievements_completed: 2,
        }
    }
}
