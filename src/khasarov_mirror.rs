use axum::{routing::get, Router, response::IntoResponse};
use crate::templates::HtmlTemplate;
use crate::templates::KhasarovMirrorTemplate;
use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new().route("/", get(khasarov_mirror_page))
}

async fn khasarov_mirror_page() -> impl IntoResponse {
    let tpl = KhasarovMirrorTemplate;
    HtmlTemplate(tpl)
}
