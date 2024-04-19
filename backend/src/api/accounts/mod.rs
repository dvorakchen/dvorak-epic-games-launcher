mod models;

use actix_web::{post, web, Responder, Result};
use models::*;

use crate::db::DbPool;
use diesel::prelude::*;
use share::{ServerResponse, SignedInResponse};

#[post("exist")]
async fn is_account_exist(db: web::Data<DbPool>, email_body: String) -> Result<impl Responder> {
    use crate::schema::accounts::dsl::*;
    let mut conn = db.get().unwrap();

    let exist = if let Ok(_) = accounts
        .select(id)
        .filter(email.eq(email_body.trim()))
        .first::<i32>(&mut conn)
    {
        true
    } else {
        false
    };

    Ok(web::Json(exist))
}

#[post("sign_in")]
async fn sign_in(db: web::Data<DbPool>, model: web::Json<SignInModel>) -> Result<impl Responder> {
    use crate::models::Account;
    use crate::schema::accounts::dsl::*;
    let mut conn = db.get().unwrap();

    if let Ok(acc) = accounts
        .filter(email.eq(&model.email))
        .filter(password.eq(&model.password))
        .select(Account::as_select())
        .first(&mut conn)
    {
        Ok(web::Json(ServerResponse::ok(SignedInResponse {
            id: acc.id,
            username: acc.username,
        })))
    } else {
        Ok(web::Json(ServerResponse::error("password wrong")))
    }
}
