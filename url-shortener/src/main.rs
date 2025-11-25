// POST /shorten

// GET/{short_url}

use std::{collections::HashMap, sync::Mutex};

use actix_web::{App, HttpResponse, HttpServer, Result, web};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
struct ShortenRequest {
    url: String,
}

#[derive(Debug, Serialize)]
struct ShortenResponse {
    short_code: String,
    short_url: String,
}

struct AppState {
    urls: Mutex<HashMap<String, String>>,
}

fn generate_short_code() -> String {
    const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    const CODE_LEN: usize = 6;

    let mut code = String::with_capacity(CODE_LEN);
    for _ in 0..CODE_LEN {
        let idx = rand::random::<usize>() % CHARSET.len();
        code.push(CHARSET[idx] as char);
    }

    code
}

// POST /shorten
async fn shorten(
    state: web::Data<AppState>,
    body: web::Json<ShortenRequest>,
) -> Result<HttpResponse> {
    let short_code = generate_short_code();

    let mut urls = state.urls
        .lock()
        .unwrap();
    urls.insert(short_code.clone(), body.url.clone());

    Ok(HttpResponse::Created().json(ShortenResponse {
        short_code: short_code.clone(),
        short_url: format!("http://localhost:8080/{}", short_code),
    }))
}

// GET /{short_url}
async fn get_short_url(
    state: web::Data<AppState>,
    code: web::Path<String>,
) -> Result<HttpResponse> {
    let urls = state.urls
        .lock()
        .unwrap();

    match urls.get(code.as_str()) {
        Some(url) => Ok(HttpResponse::Found()
            .append_header(("Location", url.as_str()))
            .finish()),
        None => Ok(HttpResponse::NotFound()
            .json(serde_json::json!({
                "error": "Short code not found"
            }))),
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        urls: Mutex::new(HashMap::new())
    });

    HttpServer::new(move ||
        App::new()
            .app_data(app_state.clone())
            .route("/shorten", web::post().to(shorten))
            .route("/{short_url}", web::get().to(get_short_url))

    )
    .bind(("127.0.01", 8080))?
    .run()
    .await
}

