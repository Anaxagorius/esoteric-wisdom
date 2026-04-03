use axum::{routing::get, Router, response::IntoResponse};
use crate::auth::HtmlTemplate;
use crate::templates::RunesTemplate;
use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new().route("/", get(runes_page))
}

async fn runes_page() -> impl IntoResponse {
    let tpl = RunesTemplate;
    HtmlTemplate(tpl)
}
