use axum::{routing::get, Router, response::IntoResponse};
use crate::auth::HtmlTemplate;
use crate::templates::{
    EsotericPracticesTemplate,
    InitiationRitesTemplate,
    TheurgyTemplate,
    HermeticPrayerTemplate,
    InvocationEvocationTemplate,
    SacredGeometryTemplate,
    MantraVibrationTemplate,
    BreathworkTemplate,
    AstralTravelTemplate,
    DreamIncubationTemplate,
    VisionQuestsTemplate,
    IChingTemplate,
    GeomancyTemplate,
    ScryingTemplate,
    PalmistryTemplate,
};
use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(hub))
        .route("/initiation-rites", get(initiation_rites))
        .route("/theurgy", get(theurgy))
        .route("/hermetic-prayer", get(hermetic_prayer))
        .route("/invocation-evocation", get(invocation_evocation))
        .route("/sacred-geometry", get(sacred_geometry))
        .route("/mantra-vibration", get(mantra_vibration))
        .route("/breathwork", get(breathwork))
        .route("/astral-travel", get(astral_travel))
        .route("/dream-incubation", get(dream_incubation))
        .route("/vision-quests", get(vision_quests))
        .route("/i-ching", get(i_ching))
        .route("/geomancy", get(geomancy))
        .route("/scrying", get(scrying))
        .route("/palmistry", get(palmistry))
}

async fn hub() -> impl IntoResponse {
    HtmlTemplate(EsotericPracticesTemplate)
}

async fn initiation_rites() -> impl IntoResponse {
    HtmlTemplate(InitiationRitesTemplate)
}

async fn theurgy() -> impl IntoResponse {
    HtmlTemplate(TheurgyTemplate)
}

async fn hermetic_prayer() -> impl IntoResponse {
    HtmlTemplate(HermeticPrayerTemplate)
}

async fn invocation_evocation() -> impl IntoResponse {
    HtmlTemplate(InvocationEvocationTemplate)
}

async fn sacred_geometry() -> impl IntoResponse {
    HtmlTemplate(SacredGeometryTemplate)
}

async fn mantra_vibration() -> impl IntoResponse {
    HtmlTemplate(MantraVibrationTemplate)
}

async fn breathwork() -> impl IntoResponse {
    HtmlTemplate(BreathworkTemplate)
}

async fn astral_travel() -> impl IntoResponse {
    HtmlTemplate(AstralTravelTemplate)
}

async fn dream_incubation() -> impl IntoResponse {
    HtmlTemplate(DreamIncubationTemplate)
}

async fn vision_quests() -> impl IntoResponse {
    HtmlTemplate(VisionQuestsTemplate)
}

async fn i_ching() -> impl IntoResponse {
    HtmlTemplate(IChingTemplate)
}

async fn geomancy() -> impl IntoResponse {
    HtmlTemplate(GeomancyTemplate)
}

async fn scrying() -> impl IntoResponse {
    HtmlTemplate(ScryingTemplate)
}

async fn palmistry() -> impl IntoResponse {
    HtmlTemplate(PalmistryTemplate)
}
