use axum::{routing::get, Router, response::IntoResponse};
use crate::templates::HtmlTemplate;
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
    EsotericMeditationTemplate,
    EsotericAstrologyTemplate,
    EsotericTarotTemplate,
    EsotericRunesTemplate,
    EsotericNumerologyTemplate,
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
        .route("/meditation", get(esoteric_meditation))
        .route("/i-ching", get(i_ching))
        .route("/geomancy", get(geomancy))
        .route("/scrying", get(scrying))
        .route("/palmistry", get(palmistry))
        .route("/astrology", get(esoteric_astrology))
        .route("/tarot", get(esoteric_tarot))
        .route("/runes", get(esoteric_runes))
        .route("/numerology", get(esoteric_numerology))
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

async fn esoteric_meditation() -> impl IntoResponse {
    HtmlTemplate(EsotericMeditationTemplate)
}

async fn esoteric_astrology() -> impl IntoResponse {
    HtmlTemplate(EsotericAstrologyTemplate)
}

async fn esoteric_tarot() -> impl IntoResponse {
    HtmlTemplate(EsotericTarotTemplate)
}

async fn esoteric_runes() -> impl IntoResponse {
    HtmlTemplate(EsotericRunesTemplate)
}

async fn esoteric_numerology() -> impl IntoResponse {
    HtmlTemplate(EsotericNumerologyTemplate)
}
