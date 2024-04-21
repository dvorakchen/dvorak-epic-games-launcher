use crate::{db::DbPool, models::RelAccountGame};
use actix_web::{error::ErrorInternalServerError, get, web, Responder, Result};
use diesel::prelude::*;
use log::debug;
use share::{GameCover, ServerResponse};

#[get("{username}")]
async fn get_in_library_games(
    db: web::Data<DbPool>,
    path: web::Path<String>,
) -> Result<impl Responder> {
    use crate::models::{Account, Game};
    use crate::schema::accounts::{self, dsl::*};
    use crate::schema::games::{self, dsl::*};
    use crate::schema::rel_account_game;

    let username_req = path.into_inner();
    let username_req = username_req.trim();

    let mut conn = db.get().expect("cannot get DB connection");

    let account_id = accounts
        .select(accounts::id)
        .filter(accounts::username.like(username_req))
        .first::<i32>(&mut conn)
        .map_err(|_| ErrorInternalServerError(format!("account {} not exist", { username_req })))?;

    if let Ok(in_library_games) = rel_account_game::table
        .inner_join(games::table)
        .filter(rel_account_game::account_id.eq(account_id))
        .select((games::id, games::name))
        .load::<(i32, String)>(&mut conn)
    {
        Ok(web::Json(ServerResponse::ok(in_library_games)))
    } else {
        Ok(web::Json(ServerResponse::ok(vec![])))
    }

    // games
    //     .grouped_by(&current_account)
    //     .into_iter()
    //     .zip(current_account)
    //     .map(|(games, account)| (account, games))
    //     .first::<(Account, Vec<Game>)>();

    // Ok(web::Json(ServerResponse::ok(in_library_games)))
}
