use dotenvy::dotenv;
use sea_orm::Database;
use actix_web::{web, App, HttpServer};
use actix_cors::Cors;
use middleware::custom_logger::CustomLogger;
use log::{info, error};
use env_logger::Env;
use seed::seed_users;

mod entities;
mod routes;
mod utils;
mod handlers;
mod models;
mod seed;
mod middleware;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize the logger with default level `debug`
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();

    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    info!("Connecting to the database...");

    let db = match Database::connect(&database_url).await {
        Ok(connection) => {
            info!("Successfully connected to the database");
            connection
        }
        Err(e) => {
            error!("Failed to connect to the database: {:?}", e);
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Database connection failed"));
        }
    };

    // Seed the database with roles
    info!("Seeding the database...");
    if let Err(e) = seed::seed_roles(&db).await {
        error!("Failed to seed roles: {:?}", e);
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Database seeding failed",
        ));
    }
    if let Err(e) = seed::seed_users(&db).await {
        error!("Failed to seed users: {:?}", e);
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Database seeding failed",
        ));
    }

    info!("Database seeding completed.");

    info!("Starting the HTTP server on 127.0.0.1:8080");

    HttpServer::new(move || {
        let cors = Cors::default()
                        .allowed_origin("http://localhost:3000")
                        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
                        .allowed_headers(vec![
                            actix_web::http::header::AUTHORIZATION,
                            actix_web::http::header::ACCEPT,
                            actix_web::http::header::CONTENT_TYPE,
                            ])
                        .max_age(3600);
        App::new()
            .wrap(CustomLogger)
            .wrap(cors)
            .app_data(web::Data::new(db.clone()))
            .configure(routes::user_routes::configure)
            .configure(routes::chat_routes::configure)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
