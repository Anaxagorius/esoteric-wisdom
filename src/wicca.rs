use axum::{routing::get, Router, response::IntoResponse};
use crate::auth::HtmlTemplate;
use crate::templates::WiccaTemplate;
use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new().route("/", get(wicca_page))
}

async fn wicca_page() -> impl IntoResponse {
    let tpl = WiccaTemplate;
    HtmlTemplate(tpl)
}
