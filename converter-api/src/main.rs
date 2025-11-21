// POST/calculate 
// GET /convert?from=km&to=miles&value=100

use actix_web::{HttpResponse, Result, web};
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

// POST/calculate
async fn calculate(body: web::Json<CalculateRequest>) -> Result<HttpResponse> {
    let result = match body.operation.as_str() {
        "add" => body.a + body.b,
        "subtract" => body.a - body.b,
        "divide" => {
            if body.b == 0.0 {
                return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                    "error" : "can not have divisor as 0"
                })));
            }
            body.a / body.b
        },
        _ => return Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Unknown operation. Use: add, subtract, multiply, divide"
        }))),
    };

    Ok(HttpResponse::Ok().json(CalculateReponse {
        operation: body.operation.clone(),
        a: body.a,
        b: body.b,
        result,
    }))
}

fn main() {
    println!("Hello, world!");
}
