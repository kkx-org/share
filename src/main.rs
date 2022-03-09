mod controllers;
mod models;
mod repositories;
mod services;
mod utils;
mod validators;

use actix_web::{
    middleware::{Logger, NormalizePath, TrailingSlash},
    web::Data,
    App, HttpServer,
};
use dotenv::dotenv;
use sqlx::PgPool;
use std::env;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    let db_pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to database");

    HttpServer::new(move || {
        App::new()
            .configure(utils::actix::init_errors)
            .wrap(Logger::default())
            .wrap(NormalizePath::new(TrailingSlash::Always))
            .app_data(Data::new(db_pool.clone()))
            .configure(controllers::init)
    })
    .bind(env::var("LISTEN_ADDRESS").expect("LISTEN_ADDRESS is not set"))?
    .run()
    .await
}
