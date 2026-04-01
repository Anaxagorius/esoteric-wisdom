use axum::{
    routing::{get, post},
    Router,
    extract::{State, Path},
    response::IntoResponse,
    Form,
    http::{StatusCode, header, HeaderMap},
};
use serde::{Serialize, Deserialize};
use tower_cookies::Cookies;
use uuid::Uuid;
use crate::auth::HtmlTemplate;
use crate::templates::{AdminJournalTemplate, GuestJournalTemplate};
use crate::state::AppState;
use crate::admin;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JournalEntry {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub body: String,
    pub mood_happy: bool,
    pub mood_reflective: bool,
    pub mood_hopeful: bool,
    pub visible_to_guests: bool,
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
        .route("/", get(journal_home))
        .route("/reflections", get(guest_reflections))
        .route("/export", get(export_journal))
        .route("/new", post(create_entry))
        .route("/delete/{id}", post(delete_entry))
        .route("/toggle/{id}", post(toggle_visibility))
}

async fn journal_home(State(state): State<AppState>, cookies: Cookies) -> impl IntoResponse {
    let claims = match admin::get_admin_claims(&state, &cookies) {
        Some(c) => c,
        None => return (StatusCode::FOUND, [(header::LOCATION, "/admin/login")]).into_response(),
    };
    if claims.must_change_password {
        return (StatusCode::FOUND, [(header::LOCATION, "/admin/change-password")]).into_response();
    }

    let entries: Vec<JournalEntry> = state.journal_entries.read().await
        .iter()
        .cloned()
        .rev()
        .collect();

    HtmlTemplate(AdminJournalTemplate { entries, error: None }).into_response()
}

async fn guest_reflections(State(state): State<AppState>) -> impl IntoResponse {
    let entries: Vec<JournalEntry> = state.journal_entries.read().await
        .iter()
        .filter(|e| e.visible_to_guests)
        .cloned()
        .rev()
        .collect();

    HtmlTemplate(GuestJournalTemplate { entries })
}

async fn create_entry(
    State(state): State<AppState>,
    cookies: Cookies,
    Form(form): Form<NewEntryForm>,
) -> impl IntoResponse {
    if !admin::is_admin(&state, &cookies) {
        return (StatusCode::FOUND, [(header::LOCATION, "/admin/login")]).into_response();
    }

    if form.body.trim().is_empty() {
        return (StatusCode::FOUND, [(header::LOCATION, "/journal")]).into_response();
    }

    let now = chrono::Utc::now().format("%B %d, %Y · %H:%M UTC").to_string();

    let entry = JournalEntry {
        id: Uuid::new_v4(),
        user_id: Uuid::nil(), // Admin is the sole author; nil UUID used as a placeholder
        title: form.title.unwrap_or_default().trim().to_string(),
        body: form.body.trim().to_string(),
        mood_happy: form.mood_happy.is_some(),
        mood_reflective: form.mood_reflective.is_some(),
        mood_hopeful: form.mood_hopeful.is_some(),
        visible_to_guests: false,
        created_at: now,
    };

    state.journal_entries.write().await.push(entry);
    (StatusCode::FOUND, [(header::LOCATION, "/journal")]).into_response()
}

async fn delete_entry(
    State(state): State<AppState>,
    cookies: Cookies,
    Path(entry_id): Path<Uuid>,
) -> impl IntoResponse {
    if !admin::is_admin(&state, &cookies) {
        return (StatusCode::FOUND, [(header::LOCATION, "/admin/login")]).into_response();
    }

    state.journal_entries.write().await.retain(|e| e.id != entry_id);
    (StatusCode::FOUND, [(header::LOCATION, "/journal")]).into_response()
}

async fn toggle_visibility(
    State(state): State<AppState>,
    cookies: Cookies,
    Path(entry_id): Path<Uuid>,
) -> impl IntoResponse {
    if !admin::is_admin(&state, &cookies) {
        return (StatusCode::FOUND, [(header::LOCATION, "/admin/login")]).into_response();
    }

    let mut entries = state.journal_entries.write().await;
    if let Some(entry) = entries.iter_mut().find(|e| e.id == entry_id) {
        entry.visible_to_guests = !entry.visible_to_guests;
    }

    (StatusCode::FOUND, [(header::LOCATION, "/journal")]).into_response()
}

async fn export_journal(
    State(state): State<AppState>,
    cookies: Cookies,
) -> impl IntoResponse {
    if !admin::is_admin(&state, &cookies) {
        return (StatusCode::FOUND, [(header::LOCATION, "/admin/login")]).into_response();
    }

    let entries: Vec<JournalEntry> = state.journal_entries.read().await.clone();

    let now = chrono::Utc::now().format("%B %d, %Y · %H:%M UTC").to_string();
    let divider = "=".repeat(48);

    let mut output = format!(
        "\u{2726} Esoteric Wisdom \u{2014} Personal Journal \u{2726}\n\
         {divider}\n\
         Exported: {now}\n\
         Total entries: {}\n\n",
        entries.len()
    );

    for (i, entry) in entries.iter().enumerate() {
        output.push_str(&format!("{divider}\n"));
        output.push_str(&format!("Entry {} of {}\n", i + 1, entries.len()));
        output.push_str(&format!("Date: {}\n", entry.created_at));

        let mut moods: Vec<&str> = Vec::new();
        if entry.mood_happy { moods.push("Grateful"); }
        if entry.mood_reflective { moods.push("Reflective"); }
        if entry.mood_hopeful { moods.push("Hopeful"); }
        if !moods.is_empty() {
            output.push_str(&format!("Mood: {}\n", moods.join(", ")));
        }

        if !entry.title.is_empty() {
            output.push_str(&format!("Title: {}\n", entry.title));
        }

        output.push('\n');
        output.push_str(&entry.body);
        output.push_str("\n\n");
    }

    output.push_str(&format!("{divider}\n"));
    output.push_str("\u{2726} End of Journal \u{2726}\n");

    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        "text/plain; charset=utf-8".parse().unwrap(),
    );
    headers.insert(
        header::CONTENT_DISPOSITION,
        "attachment; filename=\"esoteric-journal.txt\"".parse().unwrap(),
    );

    (StatusCode::OK, headers, output).into_response()
}

