use actix_web::{web, HttpResponse};
use sea_orm::prelude::Expr;
use sea_orm::sea_query::Func;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use serde::Serialize;
use crate::entities::prelude::ChatParticipants;
use crate::entities::prelude::Chats;
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

async fn get_user_chat_names(db: &DatabaseConnection, user_id: i32) -> Vec<String> {
    use crate::entities::chat_participants::Column as CpColumn;
    // Find chat participants records for this user and join with Chats.
    match ChatParticipants::find()
        .filter(CpColumn::UserId.eq(user_id))
        .find_also_related(Chats)
        .all(db)
        .await 
    {
        Ok(records) => {
            // For each record, if the related chat exists, extract its name.
            records
                .into_iter()
                .filter_map(|(_cp, maybe_chat)| maybe_chat.map(|chat| chat.name))
                .collect()
        }
        Err(_) => Vec::new(),
    }
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
                let chat_names = get_user_chat_names(db.get_ref(), user.id).await;
                user_resp.chats = chat_names;
                users_response.push(user_resp);
            }
            HttpResponse::Ok().json(GetAllUsersResponse {
            users: users_response,
            })
        }
        Err(err) => HttpResponse::InternalServerError().json(format!("Error: {:?}", err)),
    }
    
}
