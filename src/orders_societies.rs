use axum::{routing::get, Router, response::IntoResponse};
use crate::auth::HtmlTemplate;
use crate::templates::{
    OrdersSocietiesTemplate,
    EssenesTemplate,
    PythagoreanBrotherhoodTemplate,
    EleusinianMysteriesTemplate,
    OrphicMysteriesTemplate,
    MithraicMysteriesTemplate,
    DionysianMysteriesTemplate,
    EgyptianMysterySchoolsTemplate,
};
use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(hub))
        .route("/essenes", get(essenes))
        .route("/pythagorean-brotherhood", get(pythagorean_brotherhood))
        .route("/eleusinian-mysteries", get(eleusinian_mysteries))
        .route("/orphic-mysteries", get(orphic_mysteries))
        .route("/mithraic-mysteries", get(mithraic_mysteries))
        .route("/dionysian-mysteries", get(dionysian_mysteries))
        .route("/egyptian-mystery-schools", get(egyptian_mystery_schools))
}

async fn hub() -> impl IntoResponse {
    HtmlTemplate(OrdersSocietiesTemplate)
}

async fn essenes() -> impl IntoResponse {
    HtmlTemplate(EssenesTemplate)
}

async fn pythagorean_brotherhood() -> impl IntoResponse {
    HtmlTemplate(PythagoreanBrotherhoodTemplate)
}

async fn eleusinian_mysteries() -> impl IntoResponse {
    HtmlTemplate(EleusinianMysteriesTemplate)
}

async fn orphic_mysteries() -> impl IntoResponse {
    HtmlTemplate(OrphicMysteriesTemplate)
}

async fn mithraic_mysteries() -> impl IntoResponse {
    HtmlTemplate(MithraicMysteriesTemplate)
}

async fn dionysian_mysteries() -> impl IntoResponse {
    HtmlTemplate(DionysianMysteriesTemplate)
}

async fn egyptian_mystery_schools() -> impl IntoResponse {
    HtmlTemplate(EgyptianMysterySchoolsTemplate)
}
