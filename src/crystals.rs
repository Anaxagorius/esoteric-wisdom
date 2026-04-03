use axum::{routing::get, Router, response::IntoResponse};
use crate::auth::HtmlTemplate;
use crate::templates::CrystalsTemplate;
use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new().route("/", get(crystals_page))
}

async fn crystals_page() -> impl IntoResponse {
    let tpl = CrystalsTemplate;
    HtmlTemplate(tpl)
}
