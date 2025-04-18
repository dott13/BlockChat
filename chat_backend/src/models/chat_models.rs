use sea_orm::prelude::DateTimeWithTimeZone;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateChatRequest {
    pub name: String,
}

#[derive(Serialize)]
pub struct ChatResponse{
    pub id: Uuid,
    pub name: String,
    pub author_id: i32,
    pub created_at: DateTimeWithTimeZone,
}

#[derive(Deserialize)]
pub struct InviteRequest {
    pub usernames: Vec<String>,
}

#[derive(Serialize)]
pub struct InviteResponse {
    pub invited: Vec<String>,
}