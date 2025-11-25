// POST /shorten

// GET/{short_url}

use actix_web::{App, HttpServer};
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


#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(||
        App::new()
    )
    .bind(("127.0.01", 8080))?
    .run()
    .await
}
