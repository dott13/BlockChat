use actix_web::{web, HttpResponse};
use sea_orm::prelude::Expr;
use sea_orm::sea_query::Func;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use serde::Serialize;
use crate::entities::prelude::{ChatParticipants, Chats};
use crate::entities::{users, prelude::Users};
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use jsonwebtoken::{encode, Header, EncodingKey};
use chrono::{Utc, Duration};
use crate::models::user_models::*;

// Load JWT secret key at runtime
fn get_secret() -> String {
    std::env::var("JWT_SECRET").expect("JWT_SECRET must be set")
}

async fn get_user_chat_info(db: &DatabaseConnection, user_id: i32) -> Vec<ChatInfo> {
    use crate::entities::chat_participants::Column as CpColumn;
    let mut infos = Vec::new();
    // First, get all chat participation records for the user.
    if let Ok(chat_parts) = ChatParticipants::find()
        .filter(CpColumn::UserId.eq(user_id))
        .all(db)
        .await
    {
        // For each participation record, load the chat along with its author.
        for cp in chat_parts {
            // We use find_by_id on Chats and then find_also_related on Users (the chat's author).
            if let Ok(Some((chat, maybe_author))) = Chats::find_by_id(cp.chat_id)
                .find_also_related(users::Entity)
                .one(db)
                .await
            {
                if let Some(author) = maybe_author {
                    infos.push(ChatInfo {
                        chat_name: chat.name,
                        author_id: chat.author_id,
                        author_username: author.username,
                    });
                }
            }
        }
    }
    infos
}

#[derive(Serialize)]
struct Claims {
    sub: String, // Username
    exp: usize,  // Expiration time
}

// Registration Handler
pub async fn register(
    db: web::Data<DatabaseConnection>,
    form: web::Json<RegisterUser>,
) -> HttpResponse {
    use sea_orm::ColumnTrait;

    // Check if username already exists
    if Users::find()
        .filter(users::Column::Username.eq(&form.username))
        .one(db.get_ref())
        .await
        .unwrap()
        .is_some()
    {
        return HttpResponse::Conflict().json(ResponseMessage {
            message: "Username already exists".to_string(),
        });
    }

    // Hash the password with a secure salt
    let password = form.password.as_bytes();
    let mut rng = OsRng;
    let salt = SaltString::generate(&mut rng);
    let hashed_password = Argon2::default()
        .hash_password(password, &salt)
        .expect("Failed to hash password")
        .to_string();

    // Create new user
    let new_user = users::ActiveModel {
        first_name: Set(form.first_name.clone()),
        last_name: Set(form.last_name.clone()),
        username: Set(form.username.clone()),
        password: Set(hashed_password),
        role_id: Set(Some(3)), // Assuming "user" role has id 3
        ..Default::default()
    };

    match users::Entity::insert(new_user).exec(db.get_ref()).await {
        Ok(_) => HttpResponse::Ok().json(ResponseMessage {
            message: "User registered successfully".to_string(),
        }),
        Err(err) => HttpResponse::InternalServerError().body(format!("Error: {:?}", err)),
    }
}


// Login Handler
pub async fn login(
    db: web::Data<DatabaseConnection>,
    form: web::Json<LoginUser>,
) -> HttpResponse {
    use sea_orm::ColumnTrait;

    // Find the user by username
    let user = match Users::find()
        .filter(users::Column::Username.eq(&form.username))
        .one(db.get_ref())
        .await
    {
        Ok(Some(user)) => user,
        _ => {
            return HttpResponse::Unauthorized().json(ResponseMessage {
                message: "Invalid username or password".to_string(),
            })
        }
    };

    // Verify the password
    let parsed_hash = PasswordHash::new(&user.password).expect("Invalid password hash");
    if !Argon2::default().verify_password(form.password.as_bytes(), &parsed_hash).is_ok() {
        return HttpResponse::Unauthorized().json(ResponseMessage {
            message: "Invalid username or password".to_string(),
        });
    }

    // Generate a JWT token
    let claims = Claims {
        sub: user.username.clone(),
        exp: (Utc::now() + Duration::seconds(3600)).timestamp() as usize, // 1 hour expiration
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(get_secret().as_bytes()),
    )
    .unwrap();

    HttpResponse::Ok().json(serde_json::json!({
        "message": "Login successful",
        "token": token,
    }))
}

// Get All Users Handler
pub async fn get_users(
    db: web::Data<DatabaseConnection>,
    filter: web::Query<UserFilter>
) -> HttpResponse {

    let mut query = Users::find();
    if let Some(first_name) = &filter.first_name {
        // Case-insensitive match for first name
        let pattern = format!("{}%", first_name);
        query = query.filter(
            Expr::expr(Func::lower(Expr::col(users::Column::FirstName)))
                .like(pattern.to_lowercase())
        );
    }
    if let Some(last_name) = &filter.last_name {
        // Case-insensitive match for last name
        let pattern = format!("{}%", last_name);
        query = query.filter(
            Expr::expr(Func::lower(Expr::col(users::Column::LastName)))
                .like(pattern.to_lowercase())
        );
    }
    if let Some(username) = &filter.username {
        // Case-insensitive match for username
        let pattern = format!("{}%", username);
        query = query.filter(
            Expr::expr(Func::lower(Expr::col(users::Column::Username)))
                .like(pattern.to_lowercase())
        );
    }
    
    match query.all(db.get_ref()).await {
        Ok(users) => {
            let mut users_response = Vec::new();
            for user in users {
                let mut user_resp = UserResponse::from(user.clone());
                //Query chat names for the users
                let chat_info = get_user_chat_info(db.get_ref(), user.id).await;
                if filter.chat_name.is_some() && filter.author_username.is_some() {
                    let chat_name_pattern = filter.chat_name.as_ref().unwrap().to_lowercase();
                    let author_pattern = filter.author_username.as_ref().unwrap().to_lowercase();

                     // Filter the chat_info to only include matching chats
                     let filtered_chats: Vec<ChatInfo> = chat_info.into_iter()
                     .filter(|info| {
                         info.chat_name.to_lowercase().starts_with(&chat_name_pattern) && 
                         info.author_username.to_lowercase().starts_with(&author_pattern)
                     })
                     .collect();
                 
                 // Only include this user if they have any chats that match both filters
                 if !filtered_chats.is_empty() {
                     user_resp.chats = filtered_chats;
                     users_response.push(user_resp);
                 }
                } else {
                    user_resp.chats = chat_info;
                    users_response.push(user_resp); 
                }
            }
            HttpResponse::Ok().json(GetAllUsersResponse {
                users: users_response,
            })
        }
        Err(err) => HttpResponse::InternalServerError().json(format!("Error: {:?}", err)),
    }
    
}
