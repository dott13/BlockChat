use actix_web::web;
use crate::middleware::claims::RoleGuard;
use crate::handlers::chat_handler;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/chats")
                      .route("", web::post()
                      .guard(RoleGuard::new(vec!["admin", "user"]))
                      .to(chat_handler::create_chat))   
    );
}