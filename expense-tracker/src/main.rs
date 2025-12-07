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

// FILTER BY DATE RANGE: GET /expenses/by-date?start=2024-01-01&end=2025-01-01
async fn get_expense_by_date(
    state: web::Data<AppState>,
    query: web::Query<DateRangeQuery>,
) -> Result<HttpResponse> {
    let start = query.start.as_deref().unwrap_or("1900-01-01");
    let end = query.end.as_deref().unwrap_or("2025-01-01");

    let expenses = sqlx::query_as::<_, Expense>(
        "SELECT * FROM expenses WHERE date >= $1 AND date <= $2 ORDER BY date DESC"
    )
    .bind(start)
    .bind(end)
    .fetch_all(&state.db)
    .await
    .map_err(|e| {
        eprintln!("Database error {}", e);
        actix_web::error::ErrorInternalServerError("Failed to fetch expenses")
    })?;

    Ok(HttpResponse::Ok().json(expenses))
}

// FILTER BY CATEGORY GET /expenses/by-category?category=groceries
async fn filter_by_category(
    state: web::Data<AppState>,
    query: web::Query<CategoryQuery>,
) -> Result<HttpResponse> {
    let category = match &query.category {
        Some(c) => c,
        None => return Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "error" : "category query patameter is required"
        }))),
    };


    let expenses = sqlx::query_as::<_, Expense>(
        "SELECT * FROM expenses WHERE category = $1 ORDER BY date DESC"
    )
    .bind(&category)
    .fetch_all(&state.db)
    .await
    .map_err(|e| {
        eprintln!("Database error {}", e);
        actix_web::error::ErrorInternalServerError("Failed to fetch expenses")
    })?;

    Ok(HttpResponse::Ok().json(expenses))
}

// GET TOTAL SPENDING: GET /expenses/total?category=groceries (category is optional)
async fn get_total_spending(
    state: web::Data<AppState>,
    query: web::Query<CategoryQuery>,
) -> Result<HttpResponse> {
    let total: f64 = match &query.category {
        Some(category) => {
            let result: (Option<f64>, ) = sqlx::query_as(
                "SELECT SUM(category) FROM expenses WHERE category = $1"
            )
            .bind(category)
            .fetch_one(&state.db)
            .await
            .map_err(|e| {
                eprintln!("Database error {}", e);
                actix_web::error::ErrorInternalServerError("Failed to calculate total")
            })?;

            result.0.unwrap_or(0.0)
        },
        None => {
            let result: (Option<f64>, ) = sqlx::query_as(
                "SELECT SUM(amount) FROM expenses"
            )
            .fetch_one(&state.db)
            .await
            .map_err(|e| {
                eprintln!("Database error {}", e);
                actix_web::error::ErrorInternalServerError("Failed to calculate total")
            })?;
            
            result.0.unwrap_or(0.0)

        },
    };

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "total" : total,
        "category" : query.category
    })))
}

// UPDATE: PUT /expenses/{id}
async fn update_expense(
    state: web::Data<AppState>,
    id: web::Path<Uuid>,
    body: web::Json<UpdateExpenseRequest>,
) -> Result<HttpResponse> {
    let existing = sqlx::query_as::<_, Expense>(
        "SELECT * FROM expenses WHERE id = $1"
    )
    .bind(*id)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| {
        eprintln!("Database error {}", e);
        actix_web::error::ErrorInternalServerError("Failed to fetch expense")
    })?;
    
    let mut expense = match existing {
        Some(expense) => expense,
        None => return Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": "Expense not found"
        }))),
    };
    //     amount: Option<f64>,
    //     category: Option<String>,
    //     description: Option<String>,
    //     date: Option<String>,
    if let Some(amount) = body.amount {
        expense.amount = amount;
    }

    if let Some(category) = &body.category {
        expense.category = category.clone();
    }

    if let Some(description) = &body.description {
        expense.description = description.clone();
    }

    if let Some(date) = &body.date {
        expense.date = NaiveDate::parse_from_str(date, "%Y-%m-%d")
            .map_err(|_| actix_web::error::ErrorBadRequest("Invalid date format. Use YYYY-MM-DD"))?;
    };

    sqlx::query(
        "UPDATE expenses SET AMOUNT = $1, category = $2, description = $3, date = $4 WHERE id = $5"
    )
    .bind(expense.amount)
    .bind(&expense.category)
    .bind(&expense.description)
    .bind(expense.date)
    .bind(*id)
    .execute(&state.db)
    .await
    .map_err(|e| {
        eprintln!("Database error {}", e);
        actix_web::error::ErrorInternalServerError("Failed to update expense")
    })?;
 
    Ok(HttpResponse::Ok().json(expense))
}

// DELETE /expenses/{id}
async fn delete_expense(
    state: web::Data<AppState>,
    id: web::Path<Uuid>
) -> Result<HttpResponse> {
    let result = sqlx::query(
        "DELETE FROM expenses WHERE id = $1"
    )
    .bind(*id)
    .execute(&state.db)
    .await
    .map_err(|e| {
        eprintln!("Database error {}", e);
        actix_web::error::ErrorInternalServerError("Failed to delete expense")
    })?;

    if result.rows_affected() > 0 {
        Ok(HttpResponse::NoContent().finish())
    }
    else {
        Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error" : "Expense not found"
        })))
    }
}
fn main() {
    print!("");
}
