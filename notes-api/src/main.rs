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
struct UpdateNotRequest {
    title: Option<String>,
    content: Option<String>,
}

struct AppState {
    notes: Mutex<Vec<Note>>,
}

fn main() {
    
}
