use axum::{routing::get, Router, response::IntoResponse};
use crate::templates::HtmlTemplate;
use crate::templates::Ce5Template;
use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new().route("/", get(ce5_page))
}

async fn ce5_page() -> impl IntoResponse {
    HtmlTemplate(Ce5Template)
}
