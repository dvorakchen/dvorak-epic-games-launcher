pub mod signin_signout;

use signin_signout::*;
use crate::storages::get_signed_in_user_info;

pub enum InitStatus {
    // ConnectFail,
    /// the user is not signed in
    SignedOut,
    /// the user is signed in with signed in information
    SignedIn(SignedInInfo),
}

/// try connect server and check if is signed in,
/// this function should be call at launch
pub async fn init_connect_and_sign_in() -> InitStatus {
    //  1. try connect to server
    //  2. try get user information from local
    let info = get_signed_in_user_info();
    if info.is_some() {
        return InitStatus::SignedOut;
    }
    //  3. try validate sign in status with user

    let info = info.unwrap();

    InitStatus::SignedIn(info)
}
