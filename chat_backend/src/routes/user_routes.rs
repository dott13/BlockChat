use actix_web::web;
use crate::handlers::user_handler;
use crate::middleware::claims::RoleGuard;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            // Public endpoints – no guard attached.
            .route("/register", web::post().to(user_handler::register))
            .route("/login", web::post().to(user_handler::login))
            
            // Admin-only endpoints with their own resources
            .service(
                web::resource("/all")
                    .route(web::get().to(user_handler::get_users))
            )
            .service(
                web::resource("/create")
                    .wrap(RoleGuard::new(vec!["admin"]))
                    .route(web::post().to(user_handler::create_user))
            )
            
            // Single resource for user ID operations with different role guards
            .service(
                web::resource("/{id:\\d+}")
                    .route(web::get()
                        .guard(RoleGuard::new(vec!["admin", "user"]))
                        .to(user_handler::get_user))
                    .route(web::put()
                        .guard(RoleGuard::new(vec!["admin", "user"]))
                        .to(user_handler::update_user))
                    .route(web::delete()
                        .guard(RoleGuard::new(vec!["admin"]))
                        .to(user_handler::delete_user))
            )
    );
}