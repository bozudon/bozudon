use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{http::header, web, web::Data, App, HttpServer};
use dotenv::dotenv;
use sea_orm::{Database, DatabaseConnection};
use std::env;
use std::path::Path;

mod activitypub;
mod api_ext;
mod api_v1;
mod api_v2;
mod attachments;
mod config;
mod entity;
mod model;
mod nodeinfo;
mod oauth;
mod randstr;
mod router;
mod storage;
mod well_known;

use crate::config::Config;

pub async fn initialize_database() -> DatabaseConnection {
    let db_host = std::env::var("DB_HOST").unwrap();
    let db_port = std::env::var("DB_PORT").unwrap_or("5432".to_string());
    let db_user = std::env::var("DB_USER").unwrap();
    let db_pw = std::env::var("DB_PW").unwrap();
    let db_name = std::env::var("DB_NAME").unwrap();
    let database_url = format!("postgresql://{db_user}:{db_pw}@{db_host}:{db_port}/{db_name}");

    Database::connect(database_url)
        .await
        .expect("Failed to create database connection pool")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if cfg!(debug_assertions) {
        dotenv().ok();
    }

    let database = initialize_database().await;
    // FIXME ping

    tracing_subscriber::fmt::init();

    if let Err(_) = std::env::var("RUST_LOG") {
        std::env::set_var("RUST_LOG", "info");
    }

    let app_addr = std::env::var("APP_ADDR").unwrap_or("0.0.0.0".to_string());
    let app_port = std::env::var("APP_PORT")
        .unwrap_or("8080".to_string())
        .parse::<u16>()
        .unwrap();

    let data_dir = std::env::var("DATA_DIR").unwrap_or(String::from("/attachments"));
    std::fs::create_dir_all(&data_dir).unwrap();
    let storage = storage::LocalStorage::new(Path::new(&data_dir).to_path_buf());

    HttpServer::new(move || {
        let server_name = env::var("SERVER_NAME").unwrap();
        let server_url = env::var("SERVER_URL").unwrap_or("http://127.0.0.1:8080".to_string());
        let client_url = env::var("CLIENT_URL").unwrap_or("http://127.0.0.1:3000".to_string());
        App::new()
            .wrap(Logger::default())
            .app_data(Data::new(Config {
                uri: server_name.clone(),
                title: "Bozudon".to_string(),
                short_description: "".to_string(),
                description: "".to_string(),
                email: "".to_string(),
                version: "0.1.0".to_string(),
                streaming_api: format!("wss://{}", server_name.clone()),
                server_url: server_url.clone(),
            }))
            .wrap(
                Cors::default()
                    .allowed_origin(client_url.as_str())
                    .allowed_origin(server_url.as_str())
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "PATCH", "OPTIONS"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .max_age(3600),
            )
            .app_data(web::Data::new(database.clone()))
            .app_data(web::Data::new(storage.clone()))
            .configure(router::root_router)
    })
    .bind((app_addr, app_port))?
    .run()
    .await
}
