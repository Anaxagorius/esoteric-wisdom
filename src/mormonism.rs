use crate::auth::HtmlTemplate;
use crate::state::AppState;
use crate::templates::MormonismTemplate;
use axum::{response::IntoResponse, routing::get, Router};

pub fn routes() -> Router<AppState> {
    Router::new().route("/", get(mormonism_page))
}

async fn mormonism_page() -> impl IntoResponse {
    HtmlTemplate(MormonismTemplate)
}
