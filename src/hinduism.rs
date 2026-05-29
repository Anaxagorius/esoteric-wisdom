use crate::auth::HtmlTemplate;
use crate::state::AppState;
use crate::templates::HinduismTemplate;
use axum::{response::IntoResponse, routing::get, Router};

pub fn routes() -> Router<AppState> {
    Router::new().route("/", get(hinduism_page))
}

async fn hinduism_page() -> impl IntoResponse {
    HtmlTemplate(HinduismTemplate)
}
