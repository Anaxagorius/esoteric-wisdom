use axum::{routing::get, Router, response::IntoResponse};
use crate::templates::HtmlTemplate;
use crate::templates::RemoteViewingTemplate;
use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new().route("/", get(remote_viewing_page))
}

async fn remote_viewing_page() -> impl IntoResponse {
    let tpl = RemoteViewingTemplate;
    HtmlTemplate(tpl)
}
