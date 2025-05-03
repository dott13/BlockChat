use actix_web::{web, HttpRequest, HttpResponse};
use jsonwebtoken::{decode, DecodingKey, Validation};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use uuid::Uuid;

use crate::{entities::{chat_participants, chats, users}, models::{chat_models::{ChatResponse, CreateChatRequest}, token_model::Claims}, utils::jwt::get_secret};

fn extract_username(req: &HttpRequest) -> Option<String> {
    req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer ").map(str::trim))
        .and_then(|token| {
            decode::<Claims>(
                token,
                &DecodingKey::from_secret(get_secret().as_bytes()),
                &Validation::default(),
            ).ok()
        })
        .map(|data| data.claims.sub)
}

pub async fn create_chat(
    req: HttpRequest,
    db: web::Data<DatabaseConnection>,
    body: web::Json<CreateChatRequest>,
) -> HttpResponse {
    
    let username = match extract_username(&req) {
        Some(u) => u,
        None => return HttpResponse::Unauthorized().finish(),
    };

    let author = match users::Entity::find()
        .filter(users::Column::Username.eq(username))
        .one(db.get_ref())
        .await
        .unwrap() {
            Some(u) => u,
            None => return HttpResponse::Unauthorized().finish(),
    };

    let chat_id = Uuid::new_v4();

    let new_chat = chats::ActiveModel {
        id: Set(chat_id),
        name: Set(body.name.clone()),
        author_id: Set(author.id),
        ..Default::default()
    };

    let insert_res = match chats::Entity::insert(new_chat).exec(db.get_ref()).await {
        Ok(r) => r,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .body(format!("Error creating chat: {:?}", e))
        }
    };

    let chat = match chats::Entity::find_by_id(insert_res.last_insert_id)
        .one(db.get_ref())
        .await
        .unwrap()
    {
        Some(c) => c,
        None => {
            return HttpResponse::InternalServerError()
                .body("Chat inserted but not found");
        }
    };

    let _ = chat_participants::Entity::insert(chat_participants::ActiveModel {
        chat_id: Set(insert_res.last_insert_id),
        user_id: Set(author.id),
        status: Set("accepted".to_string()),
        ..Default::default()
    })
    .exec(db.get_ref())
    .await;

    for &invitee_id in &body.invitees {
        let _ = chat_participants::Entity::insert(chat_participants::ActiveModel {
            chat_id: Set(insert_res.last_insert_id),
            user_id: Set(invitee_id),
            ..Default::default()
        })
        .exec(db.get_ref())
        .await;
    }

    HttpResponse::Ok().json(ChatResponse {
        id: chat.id,
        name: chat.name,
        author_id: chat.author_id,
        created_at: chat.created_at,
    })
}