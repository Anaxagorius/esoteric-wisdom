use axum::{routing::get, Router, response::IntoResponse};
use crate::templates::HtmlTemplate;
use crate::templates::{
    MiddleEasternEsotericismTemplate,
    SufismTemplate,
    ZoroastrianismTemplate,
    MandaeismTemplate,
    KemetismTemplate,
    MesopotamianMysteriesTemplate,
    MerkabahTemplate,
};
use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(hub))
        .route("/sufism", get(sufism))
        .route("/zoroastrianism", get(zoroastrianism))
        .route("/mandaeism", get(mandaeism))
        .route("/kemetism", get(kemetism))
        .route("/mesopotamian-mysteries", get(mesopotamian_mysteries))
        .route("/merkabah", get(merkabah))
}

async fn hub() -> impl IntoResponse {
    HtmlTemplate(MiddleEasternEsotericismTemplate)
}

async fn sufism() -> impl IntoResponse {
    HtmlTemplate(SufismTemplate)
}

async fn zoroastrianism() -> impl IntoResponse {
    HtmlTemplate(ZoroastrianismTemplate)
}

async fn mandaeism() -> impl IntoResponse {
    HtmlTemplate(MandaeismTemplate)
}

async fn kemetism() -> impl IntoResponse {
    HtmlTemplate(KemetismTemplate)
}

async fn mesopotamian_mysteries() -> impl IntoResponse {
    HtmlTemplate(MesopotamianMysteriesTemplate)
}

async fn merkabah() -> impl IntoResponse {
    HtmlTemplate(MerkabahTemplate)
}
