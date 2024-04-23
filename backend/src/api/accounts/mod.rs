mod models;

use actix_web::error::ErrorInternalServerError;
use actix_web::{post, web, Responder, Result};
use models::*;
use rusqlite::params;
use rusqlite::OptionalExtension;

use crate::db::DbPool;
use share::{ServerResponse, SignedInResponse};

#[post("exist")]
async fn is_account_exist(db: web::Data<DbPool>, email: String) -> Result<impl Responder> {
    // use crate::schema::accounts::dsl::*;
    let db = db.get().unwrap();

    let mut stmt = db
        .prepare(r#"SELECT count(*) as count FROM accounts WHERE email = ?1 COLLATE NOCASE;"#)
        .unwrap();
    let account_count = stmt
        .query_row(params![email], |row| row.get::<_, i32>(0))
        .optional()
        .unwrap();

    match account_count {
        Some(1) => Ok(web::Json(true)),
        Some(v) if v > 1 => Err(ErrorInternalServerError("Account abnormal")),
        _ => Ok(web::Json(false)),
    }
}

#[post("sign_in")]
async fn sign_in(db: web::Data<DbPool>, model: web::Json<SignInModel>) -> Result<impl Responder> {
    let db = db.get().unwrap();

    let mut stmt = db
        .prepare(
            r#"SELECT id, username FROM accounts WHERE email = ?1
and password = ?2
COLLATE NOCASE"#,
        )
        .unwrap();
    let account = stmt
        .query_row(params![model.email, model.password], |row| {
            Ok(SignedInResponse {
                id: row.get(0).unwrap(),
                username: row.get(1).unwrap(),
            })
        })
        .optional()
        .unwrap();

    if let Some(account) = account {
        Ok(web::Json(ServerResponse::ok(account)))
    } else {
        Ok(web::Json(ServerResponse::error(
            "Sorry the credentials you are using are invalid.",
        )))
    }
}
