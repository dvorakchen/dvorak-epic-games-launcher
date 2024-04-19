mod accounts;
pub use accounts::SignedInResponse;

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

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ReviewType {
    ThumbsUp,
    ThumbsDown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Review {
    pub id: String,
    pub username: String,
    pub review_type: ReviewType,
    pub content: String,
    pub datetime: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ServerResponse<T> {
    pub ok: bool,
    pub error: String,
    pub content: Option<T>,
}

impl<T> ServerResponse<T> {
    /// representing the result of response is successed, 
    /// may with the response content
    pub fn ok(content: T) -> Self {
        ServerResponse {
            ok: true,
            error: "".to_string(),
            content: Some(content),
        }
    }

    /// representing the result of response is failed, 
    /// with not the response content
    pub fn error(error: impl AsRef<str>) -> Self {
        ServerResponse {
            ok: false,
            error: error.as_ref().to_string(),
            content: None,
        }
    }
}
