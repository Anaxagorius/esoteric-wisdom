use crate::templates::HtmlTemplate;
use crate::state::AppState;
use crate::templates::{
    AlteredStatesTemplate, AlteredTimeTemplate, AnomalousGnosisTemplate, BreathworkTemplate,
    ContactStatesTemplate, DissociationTemplate, DreamTelepathyTemplate, EnhancedSensesTemplate,
    GroupConsciousnessTemplate, HighAffectTemplate, HypnagogicTemplate, KundaliniTemplate,
    LiminalTemplate, LucidDreamingTemplate, MeditationTemplate, ObeTemplate, OntologicalTemplate,
    PossessionTemplate, PsychosisAdjacentTemplate, TranceTemplate, TraumaTemplate,
    VisionaryTemplate,
};
use axum::{response::IntoResponse, routing::get, Router};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(hub))
        .route("/hypnagogic", get(hypnagogic))
        .route("/lucid-dreaming", get(lucid_dreaming))
        .route("/dream-telepathy", get(dream_telepathy))
        .route("/trauma", get(trauma))
        .route("/visionary", get(visionary))
        .route("/dissociation", get(dissociation))
        .route("/kundalini", get(kundalini))
        .route("/breathwork", get(breathwork))
        .route("/trance", get(trance))
        .route("/possession", get(possession))
        .route("/meditation", get(meditation))
        .route("/obe", get(obe))
        .route("/altered-time", get(altered_time))
        .route("/enhanced-senses", get(enhanced_senses))
        .route("/gnosis", get(gnosis))
        .route("/high-affect", get(high_affect))
        .route("/group-consciousness", get(group_consciousness))
        .route("/psychosis-adjacent", get(psychosis_adjacent))
        .route("/contact-states", get(contact_states))
        .route("/ontological", get(ontological))
        .route("/liminal", get(liminal))
}

async fn hub() -> impl IntoResponse {
    HtmlTemplate(AlteredStatesTemplate)
}

async fn hypnagogic() -> impl IntoResponse {
    HtmlTemplate(HypnagogicTemplate)
}

async fn lucid_dreaming() -> impl IntoResponse {
    HtmlTemplate(LucidDreamingTemplate)
}

async fn dream_telepathy() -> impl IntoResponse {
    HtmlTemplate(DreamTelepathyTemplate)
}

async fn trauma() -> impl IntoResponse {
    HtmlTemplate(TraumaTemplate)
}

async fn visionary() -> impl IntoResponse {
    HtmlTemplate(VisionaryTemplate)
}

async fn dissociation() -> impl IntoResponse {
    HtmlTemplate(DissociationTemplate)
}

async fn kundalini() -> impl IntoResponse {
    HtmlTemplate(KundaliniTemplate)
}

async fn breathwork() -> impl IntoResponse {
    HtmlTemplate(BreathworkTemplate)
}

async fn trance() -> impl IntoResponse {
    HtmlTemplate(TranceTemplate)
}

async fn possession() -> impl IntoResponse {
    HtmlTemplate(PossessionTemplate)
}

async fn meditation() -> impl IntoResponse {
    HtmlTemplate(MeditationTemplate)
}

async fn obe() -> impl IntoResponse {
    HtmlTemplate(ObeTemplate)
}

async fn altered_time() -> impl IntoResponse {
    HtmlTemplate(AlteredTimeTemplate)
}

async fn enhanced_senses() -> impl IntoResponse {
    HtmlTemplate(EnhancedSensesTemplate)
}

async fn gnosis() -> impl IntoResponse {
    HtmlTemplate(AnomalousGnosisTemplate)
}

async fn high_affect() -> impl IntoResponse {
    HtmlTemplate(HighAffectTemplate)
}

async fn group_consciousness() -> impl IntoResponse {
    HtmlTemplate(GroupConsciousnessTemplate)
}

async fn psychosis_adjacent() -> impl IntoResponse {
    HtmlTemplate(PsychosisAdjacentTemplate)
}

async fn contact_states() -> impl IntoResponse {
    HtmlTemplate(ContactStatesTemplate)
}

async fn ontological() -> impl IntoResponse {
    HtmlTemplate(OntologicalTemplate)
}

async fn liminal() -> impl IntoResponse {
    HtmlTemplate(LiminalTemplate)
}
