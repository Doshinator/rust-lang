use actix_web::{App, HttpResponse, HttpServer, Result, delete, get, post, put, web};
use bcrypt::{DEFAULT_COST, hash, verify};
use chrono::{DateTime, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, prelude::FromRow};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
struct GameAccount {
    id: Uuid,
    user_id: Uuid,
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
struct UpdateGameAccountRequest {
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
async fn update_account(
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
async fn delete_account(
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

// --------------- Auth request 
#[derive(Debug, Serialize,FromRow)]
struct User {
    id: Uuid,
    email: String,
    #[serde(skip_serializing)]
    password_hash: String,
    created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
struct RegisterRequest {
    email: String,
    password: String,
}

#[derive(Debug, Deserialize)]
struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Debug, Serialize)]
struct AuthReponse {
    token: String,
    user: UserResponse,
}

#[derive(Debug, Serialize)]
struct UserResponse {
    id: Uuid,
    email: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    hash(password, DEFAULT_COST)
}

fn verify_password(password: &str, hash: &str) -> Result<bool, bcrypt::BcryptError> {
    verify(password, hash)
}

fn create_token(user_id: Uuid) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .expect("valid timestamp")
        .timestamp() as usize;
    
    let claims = Claims {
        sub: user_id.to_string(),
        exp: expiration,
    };

    encode(
        &Header::default(), 
        &claims, 
        &EncodingKey::from_secret(JWT_SECRET)
    )
}

fn verify_token(token: &str) -> Result<Uuid, jsonwebtoken::errors::Error> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(JWT_SECRET),
        &Validation::default(),
    )?;

    Uuid::parse_str(&token_data.claims.sub)
        .map_err(|_| {
            jsonwebtoken::errors::ErrorKind::InvalidSubject.into()
        })
}

#[post("/auth/register")]
async fn register(
    state: web::Data<AppState>,
    body: web::Json<RegisterRequest>,
) -> Result<HttpResponse> {
    let existing: Option<User> = sqlx::query_as(
        "SELECT * FROM users WHERE email = $1"
    )
    .bind(&body.email)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| {
        eprintln!("Database error: {}", e);
        actix_web::error::ErrorInternalServerError("Failed to connect to database")
    })?;

    if existing.is_some() {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "error" : "Email already registered"
        })));
    }

    let hashed_password = hash_password(&body.password)
        .map_err(|e| {
            eprint!("Hash failed");
            actix_web::error::ErrorInternalServerError("Failed to hash password")
        })?;
    
    let user_id = Uuid::new_v4();

    sqlx::query(
        "INSERT INTO users (id, email, password_hash, created_at)
        VALUES ($1, $2, $3, NOW())"
    )
    .bind(user_id)
    .bind(&body.email)
    .bind(&hashed_password)
    .execute(&state.db)
    .await
    .map_err(|e| {
        eprint!("Database error {}", e);
        actix_web::error::ErrorInternalServerError("Failed to create user")
    })?;
    
    // generate token
    let token = create_token(user_id)
        .map_err(|e| {
            eprintln!("Token error: {}", e);
            actix_web::error::ErrorInternalServerError("Failed to generate token")
        })?;
    
    Ok(HttpResponse::Created().json(AuthReponse {
        token: token,
        user: UserResponse { 
            id: user_id,
            email: body.email.clone(),
        },
    }))
}


#[post("/auth/login")]
async fn login(
    state: web::Data<AppState>,
    body: web::Json<LoginRequest>
) -> Result<HttpResponse> {
    let existing: Option<User> = sqlx::query_as(
        "SELECT * FROM users WHERE email = $1"
    )
    .bind(&body.email)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| {
        eprintln!("Database error: {}", e);
        actix_web::error::ErrorInternalServerError("Database error")
    })?;

    let user = match existing {
        Some(u) => u,
        None => return Ok(HttpResponse::Unauthorized().json(serde_json::json!({
            "error": "Invalid email or password"
        }))),
    };

    let is_valid = verify_password(&body.password, &user.password_hash)
        .map_err(|e| {
            eprintln!("Verify error: {}", e);
            actix_web::error::ErrorInternalServerError("Failed to verify password")
        })?;
    
    if !is_valid {
        return Ok(HttpResponse::Unauthorized().json(serde_json::json!({
            "error" : "Invalid email or password"
        })));
    }

    let token = create_token(user.id)
        .map_err(|e| {
            eprintln!("Token error: {}", e);
            actix_web::error::ErrorInternalServerError("Failed to generate token")
        })?;

    Ok(HttpResponse::Ok().json(AuthReponse {
        token,
        user: UserResponse {
            id: user.id,
            email: user.email,
        }
    }))
}

const JWT_SECRET: &[u8] = b"your-secret-key-change-this-in-production";
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
        // Account routes (protect these next in feature)
        .service(create_account)
        .service(list_accounts)
        .service(get_account)
        .service(update_account)
        .service(delete_account)
        // auth route public
        .service(register)
        .service(login)
        
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
