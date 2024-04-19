use serde::{Deserialize, Serialize};

/// the response struct that sign_in
#[derive(Serialize, Deserialize, Clone)]
pub struct SignedInResponse {
    pub id: i32,
    pub username: String,
}