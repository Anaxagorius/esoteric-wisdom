use axum::{routing::get, Router, response::IntoResponse};
use crate::auth::HtmlTemplate;
use crate::templates::{
    WesternEsotericismTemplate,
    HermeticismTemplate,
    GnosticismTemplate,
    NeoplatonismTemplate,
    RosicrucianismTemplate,
    ChristianMysticismTemplate,
    TheosophyTemplate,
    AnthroposophyTemplate,
    KabbalahTemplate,
    ChristianKabbalahTemplate,
    OccultismTemplate,
};
use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(hub))
        .route("/hermeticism", get(hermeticism))
        .route("/gnosticism", get(gnosticism))
        .route("/neoplatonism", get(neoplatonism))
        .route("/rosicrucianism", get(rosicrucianism))
        .route("/christian-mysticism", get(christian_mysticism))
        .route("/theosophy", get(theosophy))
        .route("/anthroposophy", get(anthroposophy))
        .route("/kabbalah", get(kabbalah))
        .route("/christian-kabbalah", get(christian_kabbalah))
        .route("/occultism", get(occultism))
}

async fn hub() -> impl IntoResponse {
    HtmlTemplate(WesternEsotericismTemplate)
}

async fn hermeticism() -> impl IntoResponse {
    HtmlTemplate(HermeticismTemplate)
}

async fn gnosticism() -> impl IntoResponse {
    HtmlTemplate(GnosticismTemplate)
}

async fn neoplatonism() -> impl IntoResponse {
    HtmlTemplate(NeoplatonismTemplate)
}

async fn rosicrucianism() -> impl IntoResponse {
    HtmlTemplate(RosicrucianismTemplate)
}

async fn christian_mysticism() -> impl IntoResponse {
    HtmlTemplate(ChristianMysticismTemplate)
}

async fn theosophy() -> impl IntoResponse {
    HtmlTemplate(TheosophyTemplate)
}

async fn anthroposophy() -> impl IntoResponse {
    HtmlTemplate(AnthroposophyTemplate)
}

async fn kabbalah() -> impl IntoResponse {
    HtmlTemplate(KabbalahTemplate)
}

async fn christian_kabbalah() -> impl IntoResponse {
    HtmlTemplate(ChristianKabbalahTemplate)
}

async fn occultism() -> impl IntoResponse {
    HtmlTemplate(OccultismTemplate)
}
