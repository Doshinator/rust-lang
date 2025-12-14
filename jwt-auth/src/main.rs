// POST /accounts - Add game account
// GET /accounts - List all accounts
// PUT /accounts/{id} - Update account
// DELETE /accounts/{id} - Delete account

use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
struct GameAccount {
    id: Uuid,
    username: String,
    platform: String,
    level: i32,
    total_hours_played: i32,
}

#[derive(Debug, Deserialize)]
struct CreateGameAccountRequest {
    username: String,
    platform: String,
}

fn main() {
    println!("Hello, world!");
}
