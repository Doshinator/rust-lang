use actix_web::{web, App, HttpServer, HttpResponse, Result};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use uuid::Uuid;
use chrono::{DateTime, Utc};

// POST /notes - Create a note
// GET /notes - List all notes
// GET /notes/{id} - Get a specific note
// PUT /notes/{id} - Update a note
// DELETE /notes/{id} - Delete a note

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Note {
    id: Uuid,
    title: String,
    content: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
struct CreateNoteRequest {
    title: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct UpdateNoteRequest {
    title: Option<String>,
    content: Option<String>,
}

struct AppState {
    notes: Mutex<Vec<Note>>,
}

// POST /notes
async fn create_note(
    state: web::Data<AppState>,
    body: web::Json<CreateNoteRequest>,
) -> Result<HttpResponse> {
    let note = Note {
        id: Uuid::new_v4(),
        title: body.title.clone(),
        content: body.content.clone(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    let mut notes = state.notes
        .lock()
        .unwrap();
    notes.push(note.clone());

    Ok(HttpResponse::Created().json(note))
}

// GET /notes
async fn list_notes(state: web::Data<AppState>) -> Result<HttpResponse> {
    let notes = state.notes.lock().unwrap();
    Ok(HttpResponse::Ok().json(&*notes))
}

// GET /notes/{id}
async fn get_note_id(
    state: web::Data<AppState>,
    id: web::Path<Uuid>,
) -> Result<HttpResponse> {
    let notes = state.notes.lock().unwrap();
    match notes.iter().find(|n| n.id == *id) {
        Some(note) => Ok(HttpResponse::Ok().json(note)),
        None => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error" : "Note not found"
        }))),
    }
}

// PUT /notes/{id} - Update a note
async fn update_note(
    state: web::Data<AppState>,
    id: web::Path<Uuid>,
    body: web::Json<UpdateNoteRequest>,
) -> Result<HttpResponse> {
    let mut notes = state.notes.lock().unwrap();
    match notes.iter_mut().find(|n| n.id == *id) {
        Some(note) => {
            if let Some(title) = &body.title {
                note.title = title.clone();
            }

            if let Some(content) = &body.content {
                note.content = content.clone();
            }
            note.updated_at = Utc::now();
            
            Ok(HttpResponse::Ok().json(note))
        },
        None => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error" : "Note not found"
        }))),
    }
}

// DELETE /notes/{id} 
async fn delete_note(
    state: web::Data<AppState>,
    id: web::Path<Uuid>,
) -> Result<HttpResponse> {
    let mut notes = state.notes.lock().unwrap();
    let original_len = notes.len();
    notes.retain(|n| n.id != *id);

    if notes.len() < original_len {
        Ok(HttpResponse::NoContent().finish())
    }
    else {
        Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error" : "Note not found"
        })))
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        notes: Mutex::new(Vec::new())
    });

    Ok(())
}
