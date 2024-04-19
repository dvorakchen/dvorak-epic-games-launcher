use std::env;

use diesel::r2d2::ConnectionManager;
use diesel::SqliteConnection;
use r2d2::Pool;

pub type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub fn get_connection_pool() -> DbPool {
    log::info!("Connecting to the database.");
    let db_path = env::var("DATABASE_URL").expect("did not set DATABASE_URL");
    let manager = ConnectionManager::<SqliteConnection>::new(db_path);

    Pool::builder()
        .build(manager)
        .expect("Failed to create connection Pool")
}
