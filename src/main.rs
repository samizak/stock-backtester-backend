use actix_web::{App, HttpServer};
use routes::configure;
use utils::cors::config;

mod handlers;
mod models;
mod routes;
mod services;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().wrap(config()).configure(configure))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
