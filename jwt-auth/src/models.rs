use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use sqlx::PgPool;
use uuid::Uuid;

// ============= Game Account Models =============
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct GameAccount {
    pub id: Uuid,
    pub user_id: Uuid,
    pub username: String,
    pub platform: String,
    pub level: i32,
    pub total_hours_played: i32,
}

#[derive(Debug, Deserialize)]
pub struct CreateGameAccountRequest {
    pub username: String,
    pub platform: String,
    pub level: Option<i32>,
    pub total_hours_played: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateGameAccountRequest {
    pub username: Option<String>,
    pub platform: Option<String>,
    pub level: Option<i32>,
    pub total_hours_played: Option<i32>,    
}

// ============= User & Auth Models =============
#[derive(Debug, Serialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: UserResponse,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

// ============= App State =============
pub struct AppState {
    pub db: PgPool,
}