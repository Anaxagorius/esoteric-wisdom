use axum::{routing::get, Router, response::IntoResponse};
use crate::auth::HtmlTemplate;
use crate::templates::ShamanismTemplate;
use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new().route("/", get(shamanism_page))
}

async fn shamanism_page() -> impl IntoResponse {
    let tpl = ShamanismTemplate;
    HtmlTemplate(tpl)
}
