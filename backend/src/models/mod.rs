use diesel::prelude::*;
use serde::Serialize;

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::accounts)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Account {
    pub id: i32,
    pub email: String,
    pub password: String,
    pub username: String,
}

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::games)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Game {
    pub id: i32,
    pub name: String,
    pub review_label: String,
    pub summary: String,
    pub score: i32,
    pub detail: String,
    pub rating_system: String,
    pub rating: String,
    pub developer: String,
    pub publisher: String,
    pub release_date: String,
    pub platform: String,
    pub create_date_time: String,
}

#[derive(Queryable, Selectable, Associations, Serialize)]
#[diesel(belongs_to(Account))]
#[diesel(belongs_to(Game))]
#[diesel(table_name = crate::schema::rel_account_game)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct RelAccountGame {
    pub account_id: i32,
    pub game_id: i32,
    pub unlock_achievement_count: i32,
}
