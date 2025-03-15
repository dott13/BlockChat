use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::entities::users;
use base64::{engine::general_purpose, Engine as _};

#[derive(Serialize)]
pub struct ChatInfo {
    pub chat_name: String,
    pub author_id: i32,
    pub author_username: String,
}

// Response struct for a user (without sensitive data).
#[derive(Serialize)]
pub struct UserResponse {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub role_id: Option<i32>,
    pub avatar: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub chats: Vec<ChatInfo>,
}

impl From<users::Model> for UserResponse {
    fn from(user: users::Model) -> Self {
        Self {
            id: user.id,
            first_name: user.first_name,
            last_name: user.last_name,
            username: user.username,
            role_id: user.role_id,
            avatar: user.avatar.map(|data| general_purpose::STANDARD.encode(data)),
            created_at: user.created_at.map(|dt| dt.with_timezone(&Utc)),
            chats: Vec::new(),
        }
    }
}


#[derive(Serialize)]
pub struct GetAllUsersResponse {
    pub users: Vec<UserResponse>,
}

#[derive(Deserialize)]
pub struct UserFilter {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub username: Option<String>,
    pub chat_name: Option<String>,
    pub author_username: Option<String>,
}

#[derive(Deserialize)]
pub struct CreateUser {
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub password: String,
    pub role_id: Option<i32>,
    pub avatar: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateUser {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub username: Option<String>,
    pub role_id: Option<i32>,
    pub avatar: Option<String>,
    pub password: Option<String>,
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
