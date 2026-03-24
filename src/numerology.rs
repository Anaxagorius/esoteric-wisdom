use axum::{routing::get, Router, response::IntoResponse};
use crate::auth::HtmlTemplate;
use crate::templates::NumerologyTemplate;
use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new().route("/", get(numerology_page))
}

async fn numerology_page() -> impl IntoResponse {
    let tpl = NumerologyTemplate;
    HtmlTemplate(tpl)
}
