mod account;

use account::{check_account, index};
use actix_web::web;

pub fn init_api(cfg: &mut web::ServiceConfig) {
    cfg.service((check_account, index));
}
