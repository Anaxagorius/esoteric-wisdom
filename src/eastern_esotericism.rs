use axum::{routing::get, Router, response::IntoResponse};
use crate::auth::HtmlTemplate;
use crate::templates::{
    EasternEsotericismTemplate,
    TantraTemplate,
    VajrayanaTemplate,
    DzogchenTemplate,
    MahamudraTemplate,
    TaoistInnerAlchemyTemplate,
    EsotericConfucianismTemplate,
    ChineseReligionTemplate,
    ShugendoTemplate,
    ShingonTemplate,
    KashmirShaivismTemplate,
    SiddhaTemplate,
};
use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(hub))
        .route("/tantra", get(tantra))
        .route("/vajrayana", get(vajrayana))
        .route("/dzogchen", get(dzogchen))
        .route("/mahamudra", get(mahamudra))
        .route("/taoist-inner-alchemy", get(taoist_inner_alchemy))
        .route("/esoteric-confucianism", get(esoteric_confucianism))
        .route("/chinese-religion", get(chinese_religion))
        .route("/shugendo", get(shugendo))
        .route("/shingon", get(shingon))
        .route("/kashmir-shaivism", get(kashmir_shaivism))
        .route("/siddha", get(siddha))
}

async fn hub() -> impl IntoResponse {
    HtmlTemplate(EasternEsotericismTemplate)
}

async fn tantra() -> impl IntoResponse {
    HtmlTemplate(TantraTemplate)
}

async fn vajrayana() -> impl IntoResponse {
    HtmlTemplate(VajrayanaTemplate)
}

async fn dzogchen() -> impl IntoResponse {
    HtmlTemplate(DzogchenTemplate)
}

async fn mahamudra() -> impl IntoResponse {
    HtmlTemplate(MahamudraTemplate)
}

async fn taoist_inner_alchemy() -> impl IntoResponse {
    HtmlTemplate(TaoistInnerAlchemyTemplate)
}

async fn esoteric_confucianism() -> impl IntoResponse {
    HtmlTemplate(EsotericConfucianismTemplate)
}

async fn chinese_religion() -> impl IntoResponse {
    HtmlTemplate(ChineseReligionTemplate)
}

async fn shugendo() -> impl IntoResponse {
    HtmlTemplate(ShugendoTemplate)
}

async fn shingon() -> impl IntoResponse {
    HtmlTemplate(ShingonTemplate)
}

async fn kashmir_shaivism() -> impl IntoResponse {
    HtmlTemplate(KashmirShaivismTemplate)
}

async fn siddha() -> impl IntoResponse {
    HtmlTemplate(SiddhaTemplate)
}
