// POST /todos - Create a todo
// GET /todos - List all todos
// GET /todos/{id} - Get one todo
// PUT /todos/{id} - Update a todo (mark as completed, or change task)
// DELETE /todos/{id} - Delete a todo

use std::sync::Mutex;

use actix_web::{App, HttpResponse, HttpServer, Result, web};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Note {
    id: Uuid,
    title: String,
    description: String,
    is_complete: bool,
}

#[derive(Debug, Deserialize)]
struct CreateNoteRequest {
    title: String,
    description: String,
}

#[derive(Debug, Deserialize)]
struct UpdateNoteRequest {
    title: Option<String>,
    description: Option<String>,
    is_complete: Option<bool>
}

async fn create_note(
    state: web::Data<AppState>,
    body: web::Json<CreateNoteRequest>,
) -> Result<HttpResponse> {
    let mut notes = state.notes
        .lock()
        .unwrap();

    let note = Note {
        id: Uuid::new_v4(),
        title: body.title.clone(),
        description: body.description.clone(),
        is_complete: false,
    };

    notes.push(note.clone());

    Ok(HttpResponse::Created().json(note))
}

async fn list_notes(state: web::Data<AppState>) -> Result<HttpResponse>{
    let notes = state.notes
        .lock()
        .unwrap();

    Ok(HttpResponse::Ok().json(&*notes))
}

async fn get_note(
    state: web::Data<AppState>,
    id: web::Path<Uuid>,
) -> Result<HttpResponse> {
    let notes = state.notes.lock().unwrap();
    match notes.iter().find(|note| note.id == *id) {
        Some(movie) => Ok(HttpResponse::Ok().json(movie)),
        None => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error" : "No todo found"
        }))),
    }
}

async fn update_todo(
    state: web::Data<AppState>,
    id: web::Path<Uuid>,
    body: web::Json<UpdateNoteRequest>,
) -> Result<HttpResponse> {
    let mut notes = state.notes.lock().unwrap();

    match notes.iter_mut().find(|note| note.id == *id) {
        Some(note) => {
            if let Some(title) = &body.title {
                note.title = title.clone();
            }
            
            if let Some(description) = &body.description {
                note.description = description.clone();
            }

            if let Some(complete) = body.is_complete {
                note.is_complete = complete;
            }

            Ok(HttpResponse::Ok().json(note))
        },
        None => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error" : "Todo not found"
        }))),
    }
}

async fn delete_todo(
    state: web::Data<AppState>,
    id: web::Path<Uuid>,
) -> Result<HttpResponse> {
    let mut notes = state.notes.lock().unwrap();
    let original_len = notes.len();
    notes.retain(|note| note.id != *id);

    if notes.len() < original_len {
        Ok(HttpResponse::NoContent().finish())
    }
    else {
        Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error" : "Note not found"
        })))
    }
}

struct AppState {
    notes : Mutex<Vec<Note>>,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(
        AppState { 
            notes: Mutex::new(Vec::new())
        }
    );

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/todos", web::post().to(create_note))
            .route("/todos", web::get().to(list_notes))
            .route("/todos/{id}", web::get().to(get_note))
            .route("/todos/{id}", web::put().to(update_todo))
            .route("/todos/{id}", web::delete().to(delete_todo))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
