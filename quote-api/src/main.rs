use std::sync::Mutex;

use actix_web::{web, App, HttpServer, HttpResponse, Result};
use rand::random;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/**
 *  POST /quotes - add a new quote
    GET /quotes/random - get a random quote
    GET /quotes - list all quotes
 */


#[derive(Debug, Clone, Serialize, Deserialize)]
struct Quote {
    id: Uuid,
    text: String,
    author: String,
}

#[derive(Debug, Deserialize)]
struct NewQuote {
    text: String,
    author: String,
}

struct AppState {
    quotes: Mutex<Vec<Quote>>,
}

// POST/quotes
async fn add_quote(
    state: web::Data<AppState>,
    new_quote: web::Json<NewQuote>,
) -> Result<HttpResponse> {
    let quote = Quote {
        id: Uuid::new_v4(),
        text: new_quote.text.clone(),
        author: new_quote.author.clone(),
    };

    let mut quotes = state.quotes
        .lock()
        .unwrap();
    quotes.push(quote.clone());

    Ok(HttpResponse::Created().json(quote))
}

// GET/quotes/random
async fn get_randon_quote(state: web::Data<AppState>) -> Result<HttpResponse> {
    let quotes = state.quotes.lock().unwrap();
    if quotes.is_empty() {
        return Ok(HttpResponse::NotFound()
            .json(serde_json::json!({
                "error": "no quotes available"
            })));
    }

    let random_index = rand::random::<usize>() % quotes.len();
    let quote = &quotes[random_index];

    Ok(HttpResponse::Ok().json(quote))
}

// GET/quotes
async fn list_quotes(state: web::Data<AppState>) -> Result<HttpResponse> {
    let quotes = state.quotes.lock().unwrap();
    Ok(HttpResponse::Ok().json(&*quotes))
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let app_state = web::Data::new(AppState {
        quotes: Mutex::new(Vec::new()),
    });

    println!("Starting Quote-Api on http://localhost:/8080");
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/quotes", web::post().to(add_quote))
            .route("/quotes/random", web::get().to(get_randon_quote))
            .route("/quotes", web::get().to(list_quotes))
    })
    .bind(("127.0.01", 8080))?
    .run()
    .await
}
