use crate::templates::HtmlTemplate;
use crate::state::AppState;
use crate::templates::JudaismTemplate;
use axum::{response::IntoResponse, routing::get, Router};

pub fn routes() -> Router<AppState> {
    Router::new().route("/", get(judaism_page))
}

async fn judaism_page() -> impl IntoResponse {
    HtmlTemplate(JudaismTemplate)
}
