// POST/calculate 
// GET /convert?from=km&to=miles&value=100

use actix_web::{App, HttpResponse, HttpServer, Result, web};
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

// GET/convert
async fn convert(query: web::Query<ConvertQuery>) -> Result<HttpResponse> {
    let converted = match (query.from.as_str(), query.to.as_str()) {
        ("km", "miles") => query.value * 0.621371,
        ("miles", "km") => query.value * 1.60934,
        ("c", "f") => (query.value * 9.0 / 5.0) + 32.0,
        ("f", "c") => (query.value - 32.0) * 5.0 / 9.0,
        ("kg", "pounds") => query.value * 2.20462,
        ("pounds", "kg") => query.value * 0.453592,
        _ => return Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "error" : "Unsupported conversion. Try: km/miles, c/f, kg/pounds",
        }))),
    };

    Ok(HttpResponse::Ok().json(ConvertQueryResponse {
        from: query.from.clone(),
        to: query.to.clone(),
        original_value: query.value,
        converted_value: converted,
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new( ||
        App::new()
        .route("/calculate", web::post().to(calculate))
        .route("/convert", web::get().to(convert))
    )
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
