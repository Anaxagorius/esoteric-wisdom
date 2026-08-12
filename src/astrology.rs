use axum::{routing::get, Router, response::IntoResponse};
use crate::templates::HtmlTemplate;
use crate::templates::AstrologyTemplate;
use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new().route("/", get(astrology_page))
}

async fn astrology_page() -> impl IntoResponse {
    let tpl = AstrologyTemplate;
    HtmlTemplate(tpl)
}
