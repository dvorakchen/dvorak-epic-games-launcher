use std::env;

use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;

pub type DbPool = Pool<SqliteConnectionManager>;

pub fn get_connection_pool() -> DbPool {
    log::info!("Connecting to the database.");
    let db_path = env::var("DATABASE_URL").expect("did not set DATABASE_URL");
    let manager = SqliteConnectionManager::file(db_path);
    r2d2::Pool::new(manager).unwrap()
}
