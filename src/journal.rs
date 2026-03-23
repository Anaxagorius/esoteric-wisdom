use axum::{
    routing::{get, post},
    Router,
    extract::{State, Path},
    response::IntoResponse,
    Form,
    http::{StatusCode, header},
};
use serde::{Serialize, Deserialize};
use tower_cookies::Cookies;
use uuid::Uuid;
use crate::auth::{HtmlTemplate, decode_token};
use crate::templates::JournalListTemplate;
use crate::state::AppState;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JournalEntry {
    pub id: Uuid,
    pub user_id: Uuid,
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
        .route("/", get(journal_home))
        .route("/new", post(create_entry))
        .route("/delete/{id}", post(delete_entry))
}

fn current_user_id(state: &AppState, cookies: &Cookies) -> Option<Uuid> {
    let token = cookies.get("esoteric_session")?.value().to_string();
    let claims = decode_token(state, &token)?;
    claims.sub.parse::<Uuid>().ok()
}

async fn journal_home(State(state): State<AppState>, cookies: Cookies) -> impl IntoResponse {
    let user_id = current_user_id(&state, &cookies);
    let is_authenticated = user_id.is_some();

    let entries = if let Some(uid) = user_id {
        let all = state.journal_entries.read().await;
        all.iter()
            .filter(|e| e.user_id == uid)
            .cloned()
            .rev()
            .collect::<Vec<_>>()
    } else {
        vec![]
    };

    let tpl = JournalListTemplate {
        is_authenticated,
        entries,
        error: None,
    };
    HtmlTemplate(tpl)
}

async fn create_entry(
    State(state): State<AppState>,
    cookies: Cookies,
    Form(form): Form<NewEntryForm>,
) -> impl IntoResponse {
    let user_id = match current_user_id(&state, &cookies) {
        Some(id) => id,
        None => {
            return (StatusCode::FOUND, [(header::LOCATION, "/auth/login")]).into_response();
        }
    };

    if form.body.trim().is_empty() {
        return (StatusCode::FOUND, [(header::LOCATION, "/journal")]).into_response();
    }

    let now = chrono::Utc::now().format("%B %d, %Y · %H:%M UTC").to_string();

    let entry = JournalEntry {
        id: Uuid::new_v4(),
        user_id,
        title: form.title.unwrap_or_default().trim().to_string(),
        body: form.body.trim().to_string(),
        mood_happy: form.mood_happy.is_some(),
        mood_reflective: form.mood_reflective.is_some(),
        mood_hopeful: form.mood_hopeful.is_some(),
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
    let user_id = match current_user_id(&state, &cookies) {
        Some(id) => id,
        None => {
            return (StatusCode::FOUND, [(header::LOCATION, "/auth/login")]).into_response();
        }
    };

    let mut entries = state.journal_entries.write().await;
    entries.retain(|e| !(e.id == entry_id && e.user_id == user_id));

    (StatusCode::FOUND, [(header::LOCATION, "/journal")]).into_response()
}
