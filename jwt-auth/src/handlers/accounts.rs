use actix_web::{delete, get, post, put, web, HttpResponse, Result};
use crate::models::{AppState, GameAccount, CreateGameAccountRequest, UpdateGameAccountRequest};
use uuid::Uuid;

#[get("/accounts")]
pub async fn list_accounts(
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
pub async fn create_account(
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
pub async fn get_account(
    state: web::Data<AppState>,
    id: web::Path<Uuid>,
) -> Result<HttpResponse> {
    let account = sqlx::query_as::<_, GameAccount>(
        "SELECT * FROM game_accounts where id = $1"
    )
    .bind(*id)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| {
        eprintln!("Database error {}", e);
        actix_web::error::ErrorInternalServerError("Failed to fetch game account")
    })?;

    match account {
        Some(account) => Ok(HttpResponse::Ok().json(account)),
        None => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "Error" : "Account not found"
        })))
    }
}

#[put("/accounts/{id}")]
pub async fn update_account(
    state: web::Data<AppState>,
    id: web::Path<Uuid>,
    body: web::Json<UpdateGameAccountRequest>
) -> Result<HttpResponse> {
    let account = sqlx::query_as::<_, GameAccount>(
        "SELECT * FROM game_accounts where id = $1"
    )
    .bind(*id)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| {
        eprintln!("Database error {}", e);
        actix_web::error::ErrorInternalServerError("Failed to fetch game account")
    })?;

    let mut account = match account {
        Some(a) => a,
        None => return Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error" : "game account not found"
        }))),
    };

    if let Some(username) = &body.username {
        account.username = username.clone();
    }

    if let Some(platform) = &body.platform {
        account.platform = platform.clone();
    }

    if let Some(lv) = body.level {
        account.level = lv;
    }

    if let Some(hours) = body.total_hours_played {
        account.total_hours_played = hours;
    }

    sqlx::query(
        "UPDATE game_accounts SET username = $1, platform = $2, level = $3, total_hours_played = $4
        WHERE id = $5"
    )
    .bind(&account.username)
    .bind(&account.platform)
    .bind(account.level)
    .bind(account.total_hours_played)
    .bind(*id)
    .execute(&state.db)
    .await
    .map_err(|e| {
        eprintln!("Database error {}", e);
        actix_web::error::ErrorInternalServerError("Failed to update game account")
    })?;

    Ok(HttpResponse::Ok().json(account))
}

#[delete("/accounts/{id}")]
pub async fn delete_account(
    state: web::Data<AppState>,
    id: web::Path<Uuid>,
) -> Result<HttpResponse> {
    let result = sqlx::query(
        "DELETE FROM game_accounts where id = $1"
    )
    .bind(*id)
    .execute(&state.db)
    .await
    .map_err(|e| {
        eprintln!("Database error {}", e);
        actix_web::error::ErrorInternalServerError("Failed to delete game account")
    })?;

    if result.rows_affected() > 0 {
        Ok(HttpResponse::NoContent().finish())
    }
    else {
        Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error" : "game account not found"
        })))
    }
}