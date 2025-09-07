use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::env;
use std::sync::Arc;

mod config;
mod models;
mod routes;
mod controllers;
mod services;

use crate::config::db::get_db_pool;
use crate::routes::product_routes::init_routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".into());
    let port = env::var("PORT").unwrap_or_else(|_| "8080".into());

    let pool = get_db_pool().await.expect("DB Pool failed");
    let pool = Arc::new(pool);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::from(pool.clone()))
            .configure(init_routes)
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
