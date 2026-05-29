use axum::{routing::get, Router, response::IntoResponse};
use crate::auth::HtmlTemplate;
use crate::templates::{
    ChristianityTemplate,
    OldTestamentTemplate,
    NewTestamentTemplate,
    KingJamesVersionTemplate,
};
use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(hub))
        .route("/old-testament", get(old_testament))
        .route("/new-testament", get(new_testament))
        .route("/king-james-version", get(king_james_version))
}

async fn hub() -> impl IntoResponse {
    HtmlTemplate(ChristianityTemplate)
}

async fn old_testament() -> impl IntoResponse {
    HtmlTemplate(OldTestamentTemplate)
}

async fn new_testament() -> impl IntoResponse {
    HtmlTemplate(NewTestamentTemplate)
}

async fn king_james_version() -> impl IntoResponse {
    HtmlTemplate(KingJamesVersionTemplate)
}
