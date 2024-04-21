mod accounts;
mod games;

use accounts::{is_account_exist, sign_in};
use actix_web::web;
use games::get_in_library_games;

pub fn init_api(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("accounts").service((is_account_exist, sign_in)))
        .service(web::scope("games").service(get_in_library_games));
}
