use actix_web::web;
use crate::handlers::user_handler;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
                    .route("/register", web::post().to(user_handler::register))
                    .route("/login", web::post().to(user_handler::login))
    );
}