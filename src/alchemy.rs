use axum::{routing::get, Router, response::IntoResponse};
use crate::templates::HtmlTemplate;
use crate::templates::AlchemyTemplate;
use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new().route("/", get(alchemy_page))
}

async fn alchemy_page() -> impl IntoResponse {
    let tpl = AlchemyTemplate;
    HtmlTemplate(tpl)
}
