use axum::{routing::get, Router, response::IntoResponse};
use crate::auth::HtmlTemplate;
use crate::templates::JournalListTemplate;
use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new().route("/", get(journal_home))
}

async fn journal_home() -> impl IntoResponse {
    let tpl = JournalListTemplate;
    HtmlTemplate(tpl)
}
