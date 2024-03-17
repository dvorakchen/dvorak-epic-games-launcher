use crate::server::signin_signout::SignedInInfo;
use gloo::storage::{LocalStorage, Storage};

const STORE_USER_INFORMATION_KEY: &'static str = "STORE_USER_INFORMATION_KEY666";

/// after signed in, need store current user information in local
pub fn save_signed_in_user_info(info: SignedInInfo) {
    LocalStorage::set(STORE_USER_INFORMATION_KEY, info).unwrap();
}

pub fn get_signed_in_user_info() -> Option<SignedInInfo> {
    LocalStorage::get::<SignedInInfo>(STORE_USER_INFORMATION_KEY).ok()
}

/// used when sign out
#[allow(dead_code)]
pub fn clear_all() {
    LocalStorage::clear();
}
