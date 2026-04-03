use axum::{routing::get, Router, response::IntoResponse};
use crate::auth::HtmlTemplate;
use crate::templates::DruidismTemplate;
use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new().route("/", get(druidism_page))
}

async fn druidism_page() -> impl IntoResponse {
    let tpl = DruidismTemplate;
    HtmlTemplate(tpl)
}
