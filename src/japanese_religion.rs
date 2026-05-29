use crate::auth::HtmlTemplate;
use crate::state::AppState;
use crate::templates::JapaneseReligionTemplate;
use axum::{response::IntoResponse, routing::get, Router};

pub fn routes() -> Router<AppState> {
    Router::new().route("/", get(japanese_religion_page))
}

async fn japanese_religion_page() -> impl IntoResponse {
    HtmlTemplate(JapaneseReligionTemplate)
}
