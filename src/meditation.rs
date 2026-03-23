use axum::{routing::get, Router, response::IntoResponse};
use crate::auth::HtmlTemplate;
use crate::templates::MeditationTemplate;
use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new().route("/", get(meditation_page))
}

async fn meditation_page() -> impl IntoResponse {
    let tpl = MeditationTemplate;
    HtmlTemplate(tpl)
}
