use axum::{routing::get, Router, response::IntoResponse};
use crate::auth::HtmlTemplate;
use crate::templates::YogaTemplate;
use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new().route("/", get(yoga_page))
}

async fn yoga_page() -> impl IntoResponse {
    HtmlTemplate(YogaTemplate)
}
