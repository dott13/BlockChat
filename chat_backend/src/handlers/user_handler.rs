use std::str;
use actix_multipart::Multipart;
use actix_web::{web, HttpRequest, HttpResponse};
use base64::engine::general_purpose;
use base64::Engine;
use futures::{StreamExt, TryStreamExt};
use sea_orm::prelude::Expr;
use sea_orm::sea_query::Func;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use crate::entities::prelude::{ChatParticipants, Chats};
use crate::entities::{users, prelude::Users};
use crate::models::token_model::Claims;
use crate::utils::check_auth_user::AuthenticatedUser;
use crate::{merge_update, merge_update_optional};
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

fn role_name_from_id(role_id: Option<i32>) -> String {
    match role_id {
        Some(2) => "admin".to_string(),
        Some(3) => "user".to_string(),
        _ => "unknown".to_string(),
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
        role: role_name_from_id(user.role_id),
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

//Create User Handler
pub async fn create_user(
    db: web::Data<DatabaseConnection>,
    req: HttpRequest,
    mut payload: web::Payload,
) -> HttpResponse {
    let content_type = req
        .headers()
        .get("Content-Type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    let mut user_data = CreateUser {
        first_name: String::new(),
        last_name: String::new(),
        username: String::new(),
        password: String::new(),
        role_id: None, // Optional – default to 3 (user) if not provided
        avatar: None,
    };

    let mut avatar_bytes: Option<Vec<u8>> = None;

    if content_type.starts_with("multipart/form-data") {
        let mut multipart = Multipart::new(req.headers(), payload);
        while let Ok(Some(mut field)) = multipart.try_next().await {
            let field_name = field
                .content_disposition()
                .and_then(|cd| cd.get_name())
                .unwrap_or("")
                .to_string();
            if field_name == "avatar" {
                let mut data = Vec::new();
                while let Some(chunk) = field.next().await {
                    match chunk {
                        Ok(bytes) => data.extend_from_slice(&bytes),
                        Err(err) => return HttpResponse::BadRequest().json(ResponseMessage {
                            message: format!("Error reading avatar image field: {:?}", err),
                        }),
                    }
                }
                avatar_bytes = Some(data);
            } else {
                let mut value = String::new();
                while let Some(chunk) = field.next().await {
                    match chunk {
                        Ok(bytes) => value.push_str(str::from_utf8(&bytes).unwrap_or("")),
                        Err(err) => return HttpResponse::BadRequest().json(ResponseMessage {
                            message: format!("Error reading field {}: {:?}", field_name, err),
                        }),
                    }
                }
                match field_name.as_str() {
                    "first_name" => user_data.first_name = value,
                    "last_name" => user_data.last_name = value,
                    "username" => user_data.username = value,
                    "password" => user_data.password = value,
                    "role_id" => {
                        if let Ok(parsed) = value.parse::<i32>() {
                            user_data.role_id = Some(parsed);
                        }
                    },
                    _=> {}
                }
            }
        }
    } else if content_type.starts_with("application/json") {
        const MAX_SIZE: usize = 262_144;
        let mut body = web::BytesMut::new();

        while let Some(chunk) = payload.next().await {
            match chunk {
                Ok(chunk) => {
                    // Limit max size of in-memory buffer
                    if (body.len() + chunk.len()) > MAX_SIZE {
                        return HttpResponse::BadRequest().json(ResponseMessage {
                            message: "Payload too large".to_string(),
                        });
                    }
                    body.extend_from_slice(&chunk);
                }
                Err(e) => {
                    return HttpResponse::BadRequest().json(ResponseMessage {
                        message: format!("Error reading payload: {:?}", e),
                    });
                }
            }
        }

        match serde_json::from_slice::<CreateUser>(&body) {
            Ok(data) => user_data = data,
            Err(e) => return HttpResponse::BadRequest().json(ResponseMessage {
                message: format!("JSON parsing error: {:?}", e),
            }),
        }
        if let Some(avatar_base64) = user_data.avatar.clone() {
            match general_purpose::STANDARD.decode(avatar_base64) {
                Ok(bytes) => avatar_bytes = Some(bytes),
                Err(e) => return HttpResponse::BadRequest().json(ResponseMessage {
                    message: format!("Invalid avatar data: {:?}", e),
                }),
            }
        } 
    } else {
        return HttpResponse::BadRequest().body("Unsupported Content-Type");
    }

     // Ensure required fields are present.
     if user_data.first_name.is_empty() ||
     user_data.last_name.is_empty() ||
     user_data.username.is_empty() ||
     user_data.password.is_empty() 
  {
      return HttpResponse::BadRequest().json(ResponseMessage {
          message: "Missing required fields".to_string(),
      });
  }

  // Check if username already exists.
  if Users::find()
      .filter(users::Column::Username.eq(&user_data.username))
      .one(db.get_ref())
      .await
      .unwrap()
      .is_some()
  {
      return HttpResponse::Conflict().json(ResponseMessage {
          message: "Username already exists".to_string(),
      });
  }

  // Hash the password.
  let password_bytes = user_data.password.as_bytes();
  let mut rng = OsRng;
  let salt = SaltString::generate(&mut rng);
  let hashed_password = Argon2::default()
      .hash_password(password_bytes, &salt)
      .expect("Failed to hash password")
      .to_string();

  // Create a new ActiveModel. If role_id is not provided, default to 3 ("user").
  let new_user_model = users::ActiveModel {
      first_name: Set(user_data.first_name),
      last_name: Set(user_data.last_name),
      username: Set(user_data.username),
      password: Set(hashed_password),
      role_id: Set(user_data.role_id.or(Some(3))),
      avatar: Set(avatar_bytes), // Will be None if not provided.
      ..Default::default()
  };

  match users::Entity::insert(new_user_model).exec(db.get_ref()).await {
      Ok(_) => HttpResponse::Ok().json(ResponseMessage {
          message: "User created successfully".to_string(),
      }),
      Err(err) => HttpResponse::InternalServerError().body(format!("Error: {:?}", err)),
  }
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

//Get a single user by id Handler
pub async fn get_user(
    db: web::Data<DatabaseConnection>,
    user_id: web::Path<i32>,
) -> HttpResponse {
    let user_id = user_id.into_inner();
    match Users::find_by_id(user_id).one(db.get_ref()).await {
        Ok(Some(user)) => {
            let mut user_resp = UserResponse::from(user);
            let chat_info = get_user_chat_info(db.get_ref(), user_resp.id).await;
            user_resp.chats = chat_info;
            HttpResponse::Ok().json(user_resp)
        }
        Ok(None) => HttpResponse::NotFound().json(ResponseMessage {
            message: "User not found".to_string()
        }),
        Err(err) => HttpResponse::InternalServerError().json(format!("Error: {:?}", err)),
    }
}

//Edit User Handler
pub async fn update_user(
    auth_user: AuthenticatedUser,
    req: HttpRequest,
    mut payload: web::Payload,
    db: web::Data<DatabaseConnection>,
) -> HttpResponse {
    // Extract user id from path.
    let user_id_str = req.match_info().get("id").unwrap_or("0");
    let user_id: i32 = match user_id_str.parse() {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().body("Invalid user id"),
    };

    // Fetch the existing user.
    let user = match Users::find_by_id(user_id).one(db.get_ref()).await {
        Ok(Some(user)) => user,
        Ok(None) => return HttpResponse::NotFound().json(ResponseMessage {
            message: "User not found".to_string(),
        }),
        Err(err) => return HttpResponse::InternalServerError().json(format!("Error: {:?}", err)),
    };
    log::debug!("Token role: {}, Token sub: {}", auth_user.0.role, auth_user.0.sub);
    // Enforce that if the authenticated user is not an admin, they can update only their own record.
    if auth_user.0.role != "admin" && auth_user.0.sub != user.username {
        return HttpResponse::Unauthorized().json(ResponseMessage {
            message: "You are not authorized to update this user".to_string(),
        });
    }

    // Convert the fetched user into an ActiveModel.
    let mut user_model: users::ActiveModel = user.into();

    // Prepare a default UpdateUser structure for JSON branch.
    let mut update_data = UpdateUser {
        first_name: None,
        last_name: None,
        username: None,
        role_id: None,
        avatar: None, // in JSON, expected as base64 string
    };
    // Variable to hold raw avatar bytes for multipart.
    let mut avatar_bytes: Option<Vec<u8>> = None;

    // Check Content-Type.
    let content_type = req
        .headers()
        .get("Content-Type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    if content_type.starts_with("multipart/form-data") {
        // Process as multipart using actix-multipart.
        let mut multipart = Multipart::new(req.headers(), payload);
        
        while let Ok(Some(mut field)) = multipart.try_next().await {
            // Extract field name.
            let field_name = field
                .content_disposition()
                .and_then(|cd| cd.get_name())
                .unwrap_or("")
                .to_string();
            
            if field_name == "avatar" {
                let mut data = Vec::new();
                while let Some(chunk) = field.next().await {
                    match chunk {
                        Ok(bytes) => data.extend_from_slice(&bytes),
                        Err(e) => return HttpResponse::BadRequest().json(ResponseMessage {
                            message: format!("Error reading avatar file: {:?}", e),
                        }),
                    }
                }
                avatar_bytes = Some(data);
            } else {
                let mut value = String::new();
                while let Some(chunk) = field.next().await {
                    match chunk {
                        Ok(bytes) => {
                            value.push_str(str::from_utf8(&bytes).unwrap_or(""));
                        }
                        Err(e) => return HttpResponse::BadRequest().json(ResponseMessage {
                            message: format!("Error reading field {}: {:?}", field_name, e),
                        }),
                    }
                }
                match field_name.as_str() {
                    "first_name" => update_data.first_name = Some(value),
                    "last_name" => update_data.last_name = Some(value),
                    "username" => update_data.username = Some(value),
                    "role_id" => {
                        if let Ok(parsed) = value.parse::<i32>() {
                            update_data.role_id = Some(parsed);
                        }
                    }
                    _ => {}
                }
            }
        }
    } else if content_type.starts_with("application/json") {
        // Process as JSON: consume the payload into bytes.
        const MAX_SIZE: usize = 262_144; // 256k limit for payload
        let mut body = web::BytesMut::new();
        
        while let Some(chunk) = payload.next().await {
            match chunk {
                Ok(chunk) => {
                    // Limit max size of in-memory buffer
                    if (body.len() + chunk.len()) > MAX_SIZE {
                        return HttpResponse::BadRequest().json(ResponseMessage {
                            message: "Payload too large".to_string(),
                        });
                    }
                    body.extend_from_slice(&chunk);
                }
                Err(e) => {
                    return HttpResponse::BadRequest().json(ResponseMessage {
                        message: format!("Error reading payload: {:?}", e),
                    });
                }
            }
        }

        match serde_json::from_slice::<UpdateUser>(&body) {
            Ok(data) => update_data = data,
            Err(e) => return HttpResponse::BadRequest().json(ResponseMessage {
                message: format!("JSON parsing error: {:?}", e),
            }),
        }
        
        // Decode avatar if provided in JSON.
        if let Some(avatar_base64) = update_data.avatar.clone() {
            match general_purpose::STANDARD.decode(avatar_base64) {
                Ok(bytes) => avatar_bytes = Some(bytes),
                Err(e) => return HttpResponse::BadRequest().json(ResponseMessage {
                    message: format!("Invalid avatar data: {:?}", e),
                }),
            }
        }
    } else {
        return HttpResponse::BadRequest().body("Unsupported Content-Type");
    }

    // Use the macros to merge fields from update_data into user_model
    merge_update!(user_model, update_data, 
        first_name,
        last_name,
        username
    );
    
    // Use the optional merge macro for nullable fields
    merge_update_optional!(user_model, update_data,
        role_id
    );
    
    // Handle avatar separately since it's processed differently
    if let Some(avatar) = avatar_bytes {
        user_model.avatar = Set(Some(avatar));
    }
    match user_model.update(db.get_ref()).await {
        Ok(_) => HttpResponse::Ok().json(ResponseMessage {
            message: "User updated successfully".to_string(),
        }),
        Err(err) => HttpResponse::InternalServerError().json(format!("Error: {:?}", err)),
    }
}

//Delete User Handler
pub async fn delete_user(
    db: web::Data<DatabaseConnection>,
    user_id: web::Path<i32>,
) -> HttpResponse {
    let user_id = user_id.into_inner();
    match Users::delete_by_id(user_id).exec(db.get_ref()).await {
        Ok(result) => {
            if result.rows_affected > 0 {
                HttpResponse::Ok().json(ResponseMessage {
                    message: "User deleted succesfully".to_string(),
                })
            } else {
                HttpResponse::NotFound().json(ResponseMessage {
                    message: "User not found".to_string(),
                })
            }
        }
        Err(err) => HttpResponse::InternalServerError().json(format!("Error: {:?}", err)),
    }
}