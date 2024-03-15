use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct User {
    id: String,
    name: String,
}

impl FromStr for User {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        _ = s;
        Ok(Self {
            id: "".to_string(),
            name: "Dvorak Chen".to_string(),
        })
    }
}
