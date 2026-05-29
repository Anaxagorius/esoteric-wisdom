use crate::auth::HtmlTemplate;
use crate::state::AppState;
use crate::templates::IslamTemplate;
use axum::{response::IntoResponse, routing::get, Router};

pub fn routes() -> Router<AppState> {
    Router::new().route("/", get(islam_page))
}

async fn islam_page() -> impl IntoResponse {
    HtmlTemplate(IslamTemplate)
}
