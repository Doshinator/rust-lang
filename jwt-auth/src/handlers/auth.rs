use actix_web::{post, web, HttpResponse, Result};
use crate::models::{AppState, RegisterRequest, LoginRequest, AuthResponse, UserResponse, User};
use crate::utils::auth::{hash_password, verify_password, create_token};
use uuid::Uuid;

#[post("/auth/register")]
pub async fn register(
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
    
    Ok(HttpResponse::Created().json(AuthResponse {
        token: token,
        user: UserResponse { 
            id: user_id,
            email: body.email.clone(),
        },
    }))
}


#[post("/auth/login")]
pub async fn login(
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

    Ok(HttpResponse::Ok().json(AuthResponse {
        token,
        user: UserResponse {
            id: user.id,
            email: user.email,
        }
    }))
}
