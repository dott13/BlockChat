use dotenvy::dotenv;
use sea_orm::Database;
use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = Database::connect(&database_url).await.expect("Failed to connect to the database");

    print!("Connected to database");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}