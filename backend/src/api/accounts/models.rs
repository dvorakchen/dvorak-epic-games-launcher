use serde::Deserialize;

#[derive(Deserialize)]
pub(super) struct SignInModel {
    pub email: String,
    pub password: String,
}
