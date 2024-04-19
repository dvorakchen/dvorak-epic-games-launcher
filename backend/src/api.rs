mod accounts;

use accounts::{is_account_exist, sign_in};
use actix_web::web;

pub fn init_api(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("accounts")
        .service((
            is_account_exist, 
            sign_in
        )));
}
