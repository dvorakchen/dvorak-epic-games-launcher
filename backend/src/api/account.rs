use actix_web::{get, post, web, Responder, Result};

use crate::db::DbPool;
use crate::models::*;
use diesel::prelude::*;

#[post("/api/accounts/account")]
async fn check_account(db: web::Data<DbPool>) -> Result<impl Responder> {
    use crate::schema::accounts::dsl::*;
    let mut conn = db.get().unwrap();

    accounts
        // .filter(email.eq(""))
        .select(Account::as_select())
        .load(&mut conn)
        .expect("Error loading accounts");

    Ok(web::Json("sss"))
}

#[get("/")]
async fn index() -> Result<impl Responder> {
    Ok(web::Json("RESPONSE"))
}
