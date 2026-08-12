use axum::{routing::get, Router, response::IntoResponse};
use crate::templates::HtmlTemplate;
use crate::templates::GatewayProcessTemplate;
use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new().route("/", get(gateway_process_page))
}

async fn gateway_process_page() -> impl IntoResponse {
    let tpl = GatewayProcessTemplate;
    HtmlTemplate(tpl)
}
