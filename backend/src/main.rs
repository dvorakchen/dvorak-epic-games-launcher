mod api;
mod db;

use actix_cors::Cors;
use actix_web::{
    middleware,
    web::{self, Data},
    App, HttpServer,
};
use dotenv::dotenv;
use log::info;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let url = env::var("DATABASE_URL");
    info!("DATABASE_URL: {:?}", url);

    let listen_address = "0.0.0.0:8080".to_string();
    info!("Listening at: {}", listen_address);

    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_header()
            .allow_any_method();

        App::new()
            .wrap(cors)
            .service(web::scope("/api").configure(api::init_api))
            // .configure(api::init_api)
            .app_data(Data::new(db::get_connection_pool()))
            .wrap(middleware::Logger::default())
    })
    .bind(listen_address)?
    .run()
    .await
}
