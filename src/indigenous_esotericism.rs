use axum::{routing::get, Router, response::IntoResponse};
use crate::auth::HtmlTemplate;
use crate::templates::{
    IndigenousEsotericismTemplate,
    AndeanCosmologyTemplate,
    NativeAmericanVisionTemplate,
    AfricanInitiatoryTemplate,
    AboriginalDreamtimeTemplate,
    HawaiianHunaTemplate,
    InuitAngakkuqTemplate,
    MayaReligionTemplate,
};
use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(hub))
        .route("/andean-cosmology", get(andean_cosmology))
        .route("/native-american-vision", get(native_american_vision))
        .route("/african-initiatory", get(african_initiatory))
        .route("/aboriginal-dreamtime", get(aboriginal_dreamtime))
        .route("/hawaiian-huna", get(hawaiian_huna))
        .route("/inuit-angakkuq", get(inuit_angakkuq))
        .route("/maya-religion", get(maya_religion))
}

async fn hub() -> impl IntoResponse {
    HtmlTemplate(IndigenousEsotericismTemplate)
}

async fn andean_cosmology() -> impl IntoResponse {
    HtmlTemplate(AndeanCosmologyTemplate)
}

async fn native_american_vision() -> impl IntoResponse {
    HtmlTemplate(NativeAmericanVisionTemplate)
}

async fn african_initiatory() -> impl IntoResponse {
    HtmlTemplate(AfricanInitiatoryTemplate)
}

async fn aboriginal_dreamtime() -> impl IntoResponse {
    HtmlTemplate(AboriginalDreamtimeTemplate)
}

async fn hawaiian_huna() -> impl IntoResponse {
    HtmlTemplate(HawaiianHunaTemplate)
}

async fn inuit_angakkuq() -> impl IntoResponse {
    HtmlTemplate(InuitAngakkuqTemplate)
}

async fn maya_religion() -> impl IntoResponse {
    HtmlTemplate(MayaReligionTemplate)
}
