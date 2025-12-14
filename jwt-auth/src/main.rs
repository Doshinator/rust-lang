// POST /accounts - Add game account
// GET /accounts - List all accounts
// PUT /accounts/{id} - Update account
// DELETE /accounts/{id} - Delete account

use actix_web::{App, HttpResponse, HttpServer, Result, get, post, web};
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

#[derive(Debug, Deserialize)]
struct UpdateGameAccount {
    username: Option<String>,
    platform: Option<String>,
    level: Option<i32>,
    total_hours_played: Option<i32>,    
}

struct AppState {
    db: PgPool,
}

#[get("/accounts")]
async fn list_accounts(
    state: web::Data<AppState>
) -> Result<HttpResponse> {
    let accounts = sqlx::query_as::<_, GameAccount>(
        "SELECT * FROM game_accounts"
    )
    .fetch_all(&state.db)
    .await
    .map_err(|e| {
        eprintln!("Connection to db failed {e}");
        actix_web::error::ErrorInternalServerError("Failed to fetch game account records")
    })?;

    Ok(HttpResponse::Ok().json(accounts))
}

#[post("/accounts")]
async fn create_account(
    state: web::Data<AppState>,
    body: web::Json<CreateGameAccountRequest>,
) -> Result<HttpResponse> {
    let id = Uuid::new_v4();
    sqlx::query(
        "INSERT into game_accounts (
            id, 
            username, 
            platform, 
            level, 
            total_hours_played
        ) 
        VALUES ($1, $2, $3, $4, $5)"
    )
    .bind(id)
    .bind(&body.username)
    .bind(&body.platform)
    .bind(body.level)
    .bind(body.total_hours_played)
    .execute(&state.db)
    .await
    .map_err(|e| {
        eprintln!("Database error {}", e);
        actix_web::error::ErrorInternalServerError("Failed to create game account")
    })?;


    Ok(HttpResponse::Created().finish())
}

#[get("/accounts/{id}")]
async fn get_account(
    state: web::Data<AppState>,
    id: web::Path<Uuid>,
) -> Result<HttpResponse> {
    let account = sqlx::query_as::<_, GameAccount>(
        "SELECT * FROM game_accounts where id = $1"
    )
    .bind(*id)
    .fetch_one(&state.db)
    .await
    .map_err(|e| {
        eprintln!("Database error {}", e);
        actix_web::error::ErrorInternalServerError("Failed to fetch game account")
    })?;

    Ok(HttpResponse::Ok().json(account))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // export DATABASE_URL="postgres://postgres:postgres@localhost/game_account_db"
    // psql -h localhost -p 5432 -U postgres -d game_account_db
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
