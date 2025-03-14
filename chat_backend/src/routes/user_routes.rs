use actix_web::web;
use crate::handlers::user_handler;
use crate::middleware::claims::RoleGuard;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            // Public endpoints (no middleware applied)
            .route("/register", web::post().to(user_handler::register))
            .route("/login", web::post().to(user_handler::login))
            // Admin-only endpoints: get all users, update and delete a user.
            .service(
                web::scope("")
                    .wrap(RoleGuard::new(vec!["admin"]))
                    .route("/all", web::get().to(user_handler::get_users))
                    .route("/{id}", web::put().to(user_handler::update_user))
                    .route("/{id}", web::delete().to(user_handler::delete_user))
            )
            // Protected endpoint: get a single user, accessible by both admin and user.
            .service(
                web::scope("")
                    .wrap(RoleGuard::new(vec!["admin", "user"]))
                    .route("/{id}", web::get().to(user_handler::get_user))
            )
    );
}
