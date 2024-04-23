use crate::db::DbPool;
use actix_web::{error::ErrorInternalServerError, get, web, Responder, Result};
use rusqlite::{params, OptionalExtension};
use share::{GameCover, ServerResponse};

#[get("{username}")]
async fn get_in_library_games(
    db: web::Data<DbPool>,
    path: web::Path<String>,
) -> Result<impl Responder> {
    let username = path.into_inner();
    let db = db.get().expect("cannot get DB connection");

    let account_id = db
        .query_row_and_then(
            r#"SELECT id FROM accounts WHERE username = ?1 COLLATE NOCASE;"#,
            params![username.trim()],
            |row| row.get::<_, i32>(0),
        )
        .optional()
        .unwrap()
        .ok_or(ErrorInternalServerError(format!("account {} not exist", {
            username
        })))?;

    let mut stmt = db.prepare(r#"SELECT 
games.id, games.name, unlock_achievement_count, games.cover_link, COUNT(achievements.id) as achievements_amount
FROM rel_account_game 
inner join games on rel_account_game.game_id = games.id
left join achievements on games.id = achievements.game_id
WHERE account_id = ?1
group by games.id, unlock_achievement_count, games.name, games.cover_link;"#).unwrap();

    let in_library_games = stmt
        .query_and_then(params![account_id], |row| {
            Ok::<_, rusqlite::Error>(GameCover {
                id: row.get_unwrap("id"),
                name: row.get_unwrap("name"),
                cover_url: row.get_unwrap("cover_link"),
                achievements_amount: row.get_unwrap("achievements_amount"),
                achievements_completed: row.get_unwrap("unlock_achievement_count"),
            })
        })
        .optional()
        .unwrap();

    if let Some(games) = in_library_games {
        let games: Vec<GameCover> = games.map_while(|game| game.ok()).collect();
        Ok(web::Json(ServerResponse::ok(games)))
    } else {
        Ok(web::Json(ServerResponse::ok(vec![])))
    }
}
