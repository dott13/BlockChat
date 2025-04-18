use actix_web::web;

use crate::handlers::chat_handler::create_chat;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/chats")
                      .route("", web::post().to(create_chat))   
    );
}