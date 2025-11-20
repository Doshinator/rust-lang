// POST/calculate 
// GET /convert?from=km&to=miles&value=100

use actix_web::web;
use serde::{Deserialize, Serialize};

// POST/calculate
#[derive(Debug, Deserialize)]
struct CalculateRequest {
    operation: String,
    a: f64,
    b: f64,
}

#[derive(Debug, Serialize)]
struct CalculateReponse {
    operation: String,
    a: f64,
    b: f64,
    result: f64,
}

// For GET /convert?from=km&to=miles&value=100
#[derive(Debug, Deserialize)]
struct ConvertQuery {
    from: String,
    to: String,
    value: f64,
}

#[derive(Debug, Serialize)]
struct ConvertQueryResponse {
    from: String,
    to: String,
    original_value: f64,
    converted_value: f64,
}

fn main() {
    println!("Hello, world!");
}
