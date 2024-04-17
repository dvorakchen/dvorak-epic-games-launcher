mod api;
mod db;
mod models;
mod schema;

use actix_cors::Cors;
use actix_web::{middleware, web::Data, App, HttpServer};
use dotenv::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    dotenv().ok();

    let listen_address = "127.0.0.1:8080".to_string();
    println!("Listening at {}", listen_address);

    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_header()
            .allow_any_method();

        App::new()
            .wrap(cors)
            .configure(api::init_api)
            .app_data(Data::new(db::get_connection_pool()))
            .wrap(middleware::Logger::default())
    })
    .bind(listen_address)?
    .run()
    .await
}
