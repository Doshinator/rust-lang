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

fn main() {
    print!("");
}
