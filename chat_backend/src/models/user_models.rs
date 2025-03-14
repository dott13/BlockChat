use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::entities::users;

// Response struct for a user (without sensitive data).
#[derive(Serialize)]
pub struct UserResponse {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub role_id: Option<i32>,
    pub created_at: Option<DateTime<Utc>>,
    pub chats: Vec<String>,
}

impl From<users::Model> for UserResponse {
    fn from(user: users::Model) -> Self {
        Self {
            id: user.id,
            first_name: user.first_name,
            last_name: user.last_name,
            username: user.username,
            role_id: user.role_id,
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
