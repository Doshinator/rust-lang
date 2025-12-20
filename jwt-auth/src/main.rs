mod models;
mod handlers;
mod middleware;
mod utils;

use actix_web::{App, HttpServer, web};
use sqlx::PgPool;
use models::AppState;

use crate::handlers::{accounts::{create_account, delete_account, get_account, list_accounts, update_account}, auth::{login, register}};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // export DATABASE_URL="postgres://postgres:postgres@localhost/game_account_db"
    // psql -h localhost -p 5432 -U postgres -d game_account_db
    let db_url = "postgres://postgres:postgres@localhost/game_account_db";
    let pool = PgPool::connect(db_url)
        .await
        .expect("Failed to connect to db");

    let app_state = web::Data::new(AppState {db: pool});

    HttpServer::new(move || {
        App::new()
        .app_data(app_state.clone())
        // Account routes (protect these next in feature)
        .service(create_account)
        .service(list_accounts)
        .service(get_account)
        .service(update_account)
        .service(delete_account)
        // auth route public
        .service(register)
        .service(login)
        
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
