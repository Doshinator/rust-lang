// POST /accounts - Add game account
// GET /accounts - List all accounts
// PUT /accounts/{id} - Update account
// DELETE /accounts/{id} - Delete account

use actix_web::{App, HttpServer, web, Result};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, prelude::FromRow};
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
    level: Option<i32>,
    total_hours_played: Option<i32>,
}

struct UpdateGameAccount {
    username: Option<String>,
    platform: Option<String>,
    level: Option<i32>,
    total_hours_played: Option<i32>,    
}

struct AppState {
    db: PgPool,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_url = "postgres://postgres:postgres@localhost/game_account_db";
    let pool = PgPool::connect(db_url)
        .await
        .expect("Failed to connect to db");

    let app_state = web::Data::new(AppState {db: pool});

    HttpServer::new(move || {
        App::new()
        .app_data(app_state.clone())
        
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
