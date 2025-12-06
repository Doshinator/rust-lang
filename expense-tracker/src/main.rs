use actix_web::{web, App, HttpServer, HttpResponse, Result};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, FromRow};
use uuid::Uuid;
use chrono::{NaiveDate, Utc};

#[derive(Debug, Serialize, FromRow)]
struct Expense {
    id: Uuid,
    amount: f64,
    category: String,
    description: String,
    date: NaiveDate,
}

#[derive(Debug, Deserialize)]
struct CreateExpenseRequest {
    amount: f64,
    category: String,
    description: String,
    date: String,
}

#[derive(Debug, Deserialize)]
struct UpdateExpenseRequest {
    amount: Option<f64>,
    category: Option<String>,
    description: Option<String>,
    date: Option<String>,
}

#[derive(Debug, Deserialize)]
struct DateRangeQuery {
    start: Option<String>,
    end: Option<String>,
}

#[derive(Debug, Deserialize)]
struct CategoryQuery {
    category: Option<String>,
}

struct AppState {
    db: PgPool,
}

// POST /expenses
async fn create_expense(
    state: web::Data<AppState>,
    body: web::Json<CreateExpenseRequest>,
) -> Result<HttpResponse> {
    let parsed_date = NaiveDate::parse_from_str(&body.date, "%Y-%m-%d")
        .map_err(|_| actix_web::error::ErrorBadRequest("Invalid date format. Use YYYY-MM-DD"))?;

    let id = Uuid::new_v4();

    sqlx::query(
        "INSERT INTO expenses (id, amount, category, description, date) VALUES ($1, $2, $3, $5, $5)"
    )
        .bind(id)
        .bind(&body.amount)
        .bind(&body.category)
        .bind(&body.description)
        .bind(&body.date)
        .execute(&state.db)
        .await
        .map_err(|e| {
            eprintln!("Database error {}", e);
            actix_web::error::ErrorInternalServerError("Failed to create expenses")
        })?;

   let expense = Expense {
        id,
        amount: body.amount,
        category: body.category.clone(),
        description: body.description.clone(),
        date: parsed_date,
    };

    Ok(HttpResponse::Created().json(expense))
}

// GET /expenses
async fn list_expenses(state: web::Data<AppState>) -> Result<HttpResponse> {
    let expenses = sqlx::query_as::<_, Expense>(
        "SELECT * FROM expenses ORDER BY date DESC"
    )
    .fetch_all(&state.db)
    .await
    .map_err(|e| {
        eprintln!("Database error {}", e);
        actix_web::error::ErrorInternalServerError("Failed to fetch all expenses")
    })?;

    Ok(HttpResponse::Ok().json(expenses))
}

// GET /expense/{id}
async fn get_expense(
    state: web::Data<AppState>,
    id: web::Path<Uuid>,
) -> Result<HttpResponse> {
    let expense = sqlx::query_as::<_, Expense>(
        "SELECT * FROM expenses where id = $1"
    )
    .bind(*id)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| {
        eprintln!("Database error", e);
        actix_web::error::ErrorInternalServerError(format!("Failed to fetch expense {}", *id))
    })?;

    match expense {
        Some(e) => Ok(HttpResponse::Ok().json(e)),
        None => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error" : "Expense not found"
        }))),
    }
}

fn main() {
    print!("");
}
