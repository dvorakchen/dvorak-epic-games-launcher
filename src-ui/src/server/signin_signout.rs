use super::get_url;
use crate::storages::{clear_all, save_signed_in_user_info};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use share::{ServerResponse, SignedInResponse};

const API_PRIFIX: &str = "accounts";

fn acconut_scope_url(path: impl AsRef<str>) -> String {
    get_url(format!("{}/{}", API_PRIFIX, path.as_ref()))
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SignedInInfo {
    pub id: i32,
    pub username: String,
    pub last_actived_datetime: DateTime<Utc>,
}

impl SignedInInfo {
    /// store current signed in user information in local storage
    pub fn store(&self) {
        save_signed_in_user_info(self.clone());
    }
}

impl From<SignedInResponse> for SignedInInfo {
    fn from(value: SignedInResponse) -> Self {
        Self {
            id: value.id,
            username: value.username,
            last_actived_datetime: Utc::now(),
        }
    }
}

pub enum SignInError {
    WrongCredential,
    WrongResponse,
}

pub async fn sign_in(
    email: impl AsRef<str>,
    password: impl AsRef<str>,
) -> Result<SignedInInfo, SignInError> {
    #[derive(Serialize)]
    struct SignInBody<'a> {
        pub email: &'a str,
        pub password: &'a str,
    }

    let email = email.as_ref();

    let client = reqwest::Client::new();
    let res = client
        .post(acconut_scope_url("sign_in"))
        .json(&SignInBody {
            email: email.as_ref(),
            password: password.as_ref(),
        })
        .send()
        .await
        .map_err(|_| SignInError::WrongCredential)?
        .json::<ServerResponse<SignedInResponse>>()
        .await
        .map_err(|_| SignInError::WrongCredential)?;

    leptos::logging::log!("response: {}", serde_json::to_string(&res).unwrap());
    gloo::timers::future::TimeoutFuture::new(1_000).await;

    match res {
        ServerResponse::<SignedInResponse> {
            ok: true,
            content: Some(content),
            ..
        } => Ok(content.into()),
        ServerResponse::<SignedInResponse> {
            ok: true,
            content: None,
            ..
        } => Err(SignInError::WrongResponse),
        ServerResponse::<SignedInResponse> { ok: false, .. } => Err(SignInError::WrongCredential),
    }
}

pub fn sign_out() {
    clear_all();
}

pub async fn check_account_exist(email: impl AsRef<str>) -> Result<(), ()> {
    let client = reqwest::Client::new();
    let res = client
        .post(get_url(format!("{}/exist", API_PRIFIX)))
        .body(email.as_ref().to_string())
        .send()
        .await
        .map_err(|_| ())?
        .json::<bool>()
        .await
        .map_err(|_| ())?;

    //  just simulating the network latency
    gloo::timers::future::TimeoutFuture::new(1_000).await;

    if res {
        Ok(())
    } else {
        Err(())
    }
}
