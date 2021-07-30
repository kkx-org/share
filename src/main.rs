mod api;
mod db;

use actix_web::{middleware::Logger, App, HttpServer};
use dotenv::dotenv;
use sqlx::PgPool;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    let db_pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to database");

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .data(db_pool.clone())
            .configure(api::init)
    })
    .bind(env::var("LISTEN_ADDRESS").expect("LISTEN_ADDRESS is not set"))?
    .run()
    .await
}
