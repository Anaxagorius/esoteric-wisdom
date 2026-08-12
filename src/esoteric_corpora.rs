use axum::{routing::get, Router, response::IntoResponse};
use crate::templates::HtmlTemplate;
use crate::templates::{
    EsotericCorporaTemplate,
    CorpusHermeticumTemplate,
    NagHammadiTemplate,
    ZoharTemplate,
    SeferYetzirahTemplate,
    SeferBahirTemplate,
    EmeraldTabletTemplate,
    UpanishadsEsotericTemplate,
    TantrasTextsTemplate,
    TibetanBookDeadTemplate,
    BookOfTheLawTemplate,
    PicatrixTemplate,
    ChaldeaOraclesTemplate,
    OrphicHymnsTemplate,
};
use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(hub))
        .route("/corpus-hermeticum", get(corpus_hermeticum))
        .route("/nag-hammadi", get(nag_hammadi))
        .route("/zohar", get(zohar))
        .route("/sefer-yetzirah", get(sefer_yetzirah))
        .route("/sefer-bahir", get(sefer_bahir))
        .route("/emerald-tablet", get(emerald_tablet))
        .route("/upanishads", get(upanishads))
        .route("/tantras", get(tantras))
        .route("/tibetan-book-dead", get(tibetan_book_dead))
        .route("/book-of-the-law", get(book_of_the_law))
        .route("/picatrix", get(picatrix))
        .route("/chaldean-oracles", get(chaldean_oracles))
        .route("/orphic-hymns", get(orphic_hymns))
}

async fn hub() -> impl IntoResponse {
    HtmlTemplate(EsotericCorporaTemplate)
}

async fn corpus_hermeticum() -> impl IntoResponse {
    HtmlTemplate(CorpusHermeticumTemplate)
}

async fn nag_hammadi() -> impl IntoResponse {
    HtmlTemplate(NagHammadiTemplate)
}

async fn zohar() -> impl IntoResponse {
    HtmlTemplate(ZoharTemplate)
}

async fn sefer_yetzirah() -> impl IntoResponse {
    HtmlTemplate(SeferYetzirahTemplate)
}

async fn sefer_bahir() -> impl IntoResponse {
    HtmlTemplate(SeferBahirTemplate)
}

async fn emerald_tablet() -> impl IntoResponse {
    HtmlTemplate(EmeraldTabletTemplate)
}

async fn upanishads() -> impl IntoResponse {
    HtmlTemplate(UpanishadsEsotericTemplate)
}

async fn tantras() -> impl IntoResponse {
    HtmlTemplate(TantrasTextsTemplate)
}

async fn tibetan_book_dead() -> impl IntoResponse {
    HtmlTemplate(TibetanBookDeadTemplate)
}

async fn book_of_the_law() -> impl IntoResponse {
    HtmlTemplate(BookOfTheLawTemplate)
}

async fn picatrix() -> impl IntoResponse {
    HtmlTemplate(PicatrixTemplate)
}

async fn chaldean_oracles() -> impl IntoResponse {
    HtmlTemplate(ChaldeaOraclesTemplate)
}

async fn orphic_hymns() -> impl IntoResponse {
    HtmlTemplate(OrphicHymnsTemplate)
}
