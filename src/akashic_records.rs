use axum::{routing::get, Router, response::IntoResponse};
use crate::templates::HtmlTemplate;
use crate::templates::AkashicRecordsTemplate;
use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new().route("/", get(akashic_records_page))
}

async fn akashic_records_page() -> impl IntoResponse {
    let tpl = AkashicRecordsTemplate;
    HtmlTemplate(tpl)
}
