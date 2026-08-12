use axum::{
    routing::{get, post},
    Router,
    extract::State,
    response::IntoResponse,
    Form,
    http::{StatusCode, header},
};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::templates::HtmlTemplate;
use crate::templates::GuestJournalTemplate;
use crate::state::{AppState, journal_data_path};

/// Blocked words list — applied to every submission. Non-negotiable.
const BLOCKED_WORDS: &[&str] = &[
    "fuck", "shit", "cunt", "nigger", "nigga", "faggot", "fag", "retard",
    "bitch", "asshole", "bastard", "dick", "cock", "pussy", "whore", "slut",
    "twat", "wanker", "prick", "arse", "damn", "crap",
    "motherfucker", "fucker", "bullshit", "jackass", "dumbass", "douchebag",
    "kike", "spic", "chink", "gook", "wetback", "cracker", "honky",
];

fn contains_blocked_language(text: &str) -> bool {
    let lower = text.to_lowercase();
    // Use word-boundary-aware check: look for the word as a standalone token
    for word in BLOCKED_WORDS {
        // Simple substring check — intentionally strict; if the word appears
        // anywhere in the text it is blocked.
        if lower.contains(word) {
            return true;
        }
    }
    false
}

async fn save_journal(entries: &[JournalEntry]) {
    let path = journal_data_path();
    match serde_json::to_string(entries) {
        Ok(json) => {
            if let Some(parent) = std::path::Path::new(&path).parent() {
                let _ = tokio::fs::create_dir_all(parent).await;
            }
            if let Err(e) = tokio::fs::write(&path, json).await {
                tracing::warn!("Failed to save journal to {path}: {e}");
            }
        }
        Err(e) => tracing::warn!("Failed to serialize journal entries: {e}"),
    }
}

/// Simplified entry format — no user tracking, all entries are public.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JournalEntry {
    pub id: Uuid,
    pub title: String,
    pub body: String,
    pub mood_happy: bool,
    pub mood_reflective: bool,
    pub mood_hopeful: bool,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct NewEntryForm {
    pub title: Option<String>,
    pub body: String,
    pub mood_happy: Option<String>,
    pub mood_reflective: Option<String>,
    pub mood_hopeful: Option<String>,
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/reflections", get(reflections))
        .route("/new", post(create_entry))
}

async fn reflections(State(state): State<AppState>) -> impl IntoResponse {
    let entries: Vec<JournalEntry> = state.journal_entries.read().await
        .iter()
        .cloned()
        .rev()
        .collect();

    HtmlTemplate(GuestJournalTemplate { entries })
}

async fn create_entry(
    State(state): State<AppState>,
    Form(form): Form<NewEntryForm>,
) -> impl IntoResponse {
    let body = form.body.trim().to_string();

    if body.is_empty() {
        return (StatusCode::FOUND, [(header::LOCATION, "/journal/reflections")]).into_response();
    }

    let title = form.title.unwrap_or_default().trim().to_string();

    // Language filter — non-negotiable
    if contains_blocked_language(&body) || contains_blocked_language(&title) {
        return (StatusCode::FOUND, [(header::LOCATION, "/journal/reflections?blocked=1")]).into_response();
    }

    let now = chrono::Utc::now().format("%B %d, %Y · %H:%M UTC").to_string();

    let entry = JournalEntry {
        id: Uuid::new_v4(),
        title,
        body,
        mood_happy: form.mood_happy.is_some(),
        mood_reflective: form.mood_reflective.is_some(),
        mood_hopeful: form.mood_hopeful.is_some(),
        created_at: now,
    };

    state.journal_entries.write().await.push(entry);
    let snapshot = state.journal_entries.read().await.clone();
    save_journal(&snapshot).await;

    (StatusCode::FOUND, [(header::LOCATION, "/journal/reflections")]).into_response()
}
