use actix_web::{web, HttpResponse};
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};
use crate::entities::{users, prelude::Users};
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use rand_core::OsRng;
use argon2::password_hash::SaltString;
use jsonwebtoken::{encode, Header, EncodingKey};
use chrono::{Utc, Duration};

// Load JWT secret key at runtime
fn get_secret() -> String {
    std::env::var("JWT_SECRET").expect("JWT_SECRET must be set")
}

// JWT Claims
#[derive(Serialize)]
struct Claims {
    sub: String, // Username
    exp: usize,  // Expiration time
}

#[derive(Deserialize)]
pub struct RegisterUser {
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginUser {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct ResponseMessage {
    pub message: String,
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
    let salt = SaltString::generate(&mut OsRng); // Generate a secure random salt
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
