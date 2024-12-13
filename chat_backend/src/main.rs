use dotenvy::dotenv;
use sea_orm::Database;
use actix_web::{web, App, HttpServer};
use log::{info, error};
use env_logger::Env;

mod entities;
mod routes;
mod handlers;
mod seed;

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
    info!("Database seeding completed.");

    info!("Starting the HTTP server on 127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .configure(routes::user_routes::configure)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
