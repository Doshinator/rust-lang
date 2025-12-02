use actix_web::{App, HttpResponse, HttpServer, Result, web};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use uuid::Uuid;

// POST /movies - Add a movie
// GET /movies - List all movies
// GET /movies/{id} - Get a specific movie
// PUT /movies/{id} - Update a movie
// DELETE /movies/{id} - Delete a movie

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Movie {
    id: Uuid,
    title: String,
    director: String,
    year: u32,
    rating: Option<f32>,
    watched: bool,
}

#[derive(Debug, Deserialize)]
struct CreateMovieRequest {
    title: String,
    director: String,
    year: u32,
    rating: Option<f32>,
}

#[derive(Debug, Deserialize)]
struct UpdateMovieRequest {
    title: Option<String>,
    director: Option<String>,
    year: Option<u32>,
    rating: Option<f32>,
    watched: Option<bool>,
}

struct AppState {
    movies: Mutex<Vec<Movie>>,
}

async fn create_movie(
    state: web::Data<AppState>,
    body: web::Json<CreateMovieRequest>,
) -> Result<HttpResponse> {
    let movie = Movie {
        id: Uuid::new_v4(),
        title: body.title.clone(),
        director: body.director.clone(),
        year: body.year,
        rating: body.rating,
        watched: false,
    };

    let mut movies = state.movies
        .lock()
        .unwrap();

    movies.push(movie);

    Ok(HttpResponse::Created().finish())
}

async fn list_movies(state: web::Data<AppState>) -> Result<HttpResponse> {
    let movies = state.movies
        .lock()
        .unwrap();

    for movie in movies.iter() {
        println!("{:?}", movie);
    }
    Ok(HttpResponse::Ok().json(&*movies))
}

async fn get_movie(
    state: web::Data<AppState>,
    id: web::Path<Uuid>,
) -> Result<HttpResponse> {
    let movies = state.movies
        .lock()
        .unwrap();

    match movies.iter().find(|n| n.id == *id) {
        Some(movie) => Ok(HttpResponse::Ok().json(movie)),
        None => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error" : "Movie not found"
        }))),
    }
}

async fn update_movie(
    state: web::Data<AppState>,
    id: web::Path<Uuid>,
    body: web::Json<UpdateMovieRequest>,
) -> Result<HttpResponse> {
    let mut movies = state.movies
        .lock()
        .unwrap();

    match movies.iter_mut().find(|n| n.id == *id) {
        Some(movie) => {
            if let Some(title) = &body.title {
                movie.title = title.clone();
            }
            
            if let Some(director) = &body.director {
                movie.director = director.clone();
            }

            if let Some(year) = &body.year {
                movie.year = year.clone();
            }

            if let Some(rating) = body.rating {
                movie.rating = Some(rating);
            }

            if let Some(watched) = body.watched {
                movie.watched = watched;
            }

            Ok(HttpResponse::Ok().json(movie))
        },
        None => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error" : "Movie not found"
        }))),
    }
}

async fn delete_movie(
    state: web::Data<AppState>,
    id: web::Path<Uuid>,
) -> Result<HttpResponse> {
    let mut movies = state.movies
        .lock()
        .unwrap();

    let original_len = movies.len();
    movies.retain(|m| m.id != *id);

    if movies.len() < original_len {
        Ok(HttpResponse::NoContent().finish())
    }
    else {
        Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error" : "Movie not found"
        })))
    }
}

#[tokio::main]
async  fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        movies: Mutex::new(Vec::new())
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/movies", web::post().to(create_movie))
            .route("/movies", web::get().to(list_movies))
            .route("/movies/{id}", web::get().to(get_movie))
            .route("/movies/{id}", web::put().to(update_movie))
            .route("/movies/{id}", web::delete().to(delete_movie))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
