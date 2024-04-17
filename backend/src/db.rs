use diesel::r2d2::ConnectionManager;
use diesel::SqliteConnection;
use r2d2::Pool;

pub type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub fn get_connection_pool() -> DbPool {
    log::info!("Connecting to the database.");
    let manager = ConnectionManager::<SqliteConnection>::new("./epic.db");

    Pool::builder()
        .build(manager)
        .expect("Failed to create connection Pool")
}
