use crate::storages::save_signed_in_user_info;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SignedInInfo {
    pub id: String,
    pub username: String,
    pub last_actived_datetime: DateTime<Utc>,
}

impl SignedInInfo {
    /// store current signed in user information in local storage
    pub fn store(&self) {
        save_signed_in_user_info(self.clone());
    }
}

pub enum SignInError {
    WrongCredential,
}

pub async fn sign_in(
    email: impl AsRef<str>,
    password: impl AsRef<str>,
) -> Result<SignedInInfo, SignInError> {
    _ = email;

    const DEBUG_PASSWORD: &'static str = "123123";
    if DEBUG_PASSWORD == password.as_ref() {
        Ok(SignedInInfo {
            id: "000001".to_string(),
            username: "DvorakChen".to_string(),
            last_actived_datetime: Utc::now(),
        })
    } else {
        Err(SignInError::WrongCredential)
    }
}
