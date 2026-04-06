use axum::{routing::get, Router, response::IntoResponse};
use crate::auth::HtmlTemplate;
use crate::templates::{
    EsotericConceptsThemesTemplate,
    GnosisTemplate,
    InitiationConceptTemplate,
    HiddenWisdomTemplate,
    InnerRevelationTemplate,
    SacredSecrecyTemplate,
    SymbolismLiteralismTemplate,
    MicrocosmMacrocosmTemplate,
    AsAboveSoBelowTemplate,
    InnerChristBuddhaNatureTemplate,
    TransmutationConsciousnessTemplate,
    AscensionIlluminationTemplate,
    SpiritualAlchemyTemplate,
};
use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(hub))
        .route("/gnosis", get(gnosis))
        .route("/initiation", get(initiation))
        .route("/hidden-wisdom", get(hidden_wisdom))
        .route("/inner-revelation", get(inner_revelation))
        .route("/sacred-secrecy", get(sacred_secrecy))
        .route("/symbolism-over-literalism", get(symbolism_over_literalism))
        .route("/microcosm-macrocosm", get(microcosm_macrocosm))
        .route("/as-above-so-below", get(as_above_so_below))
        .route("/inner-christ-buddha-nature", get(inner_christ_buddha_nature))
        .route("/transmutation-of-consciousness", get(transmutation_of_consciousness))
        .route("/ascension-illumination", get(ascension_illumination))
        .route("/spiritual-alchemy", get(spiritual_alchemy))
}

async fn hub() -> impl IntoResponse {
    HtmlTemplate(EsotericConceptsThemesTemplate)
}

async fn gnosis() -> impl IntoResponse {
    HtmlTemplate(GnosisTemplate)
}

async fn initiation() -> impl IntoResponse {
    HtmlTemplate(InitiationConceptTemplate)
}

async fn hidden_wisdom() -> impl IntoResponse {
    HtmlTemplate(HiddenWisdomTemplate)
}

async fn inner_revelation() -> impl IntoResponse {
    HtmlTemplate(InnerRevelationTemplate)
}

async fn sacred_secrecy() -> impl IntoResponse {
    HtmlTemplate(SacredSecrecyTemplate)
}

async fn symbolism_over_literalism() -> impl IntoResponse {
    HtmlTemplate(SymbolismLiteralismTemplate)
}

async fn microcosm_macrocosm() -> impl IntoResponse {
    HtmlTemplate(MicrocosmMacrocosmTemplate)
}

async fn as_above_so_below() -> impl IntoResponse {
    HtmlTemplate(AsAboveSoBelowTemplate)
}

async fn inner_christ_buddha_nature() -> impl IntoResponse {
    HtmlTemplate(InnerChristBuddhaNatureTemplate)
}

async fn transmutation_of_consciousness() -> impl IntoResponse {
    HtmlTemplate(TransmutationConsciousnessTemplate)
}

async fn ascension_illumination() -> impl IntoResponse {
    HtmlTemplate(AscensionIlluminationTemplate)
}

async fn spiritual_alchemy() -> impl IntoResponse {
    HtmlTemplate(SpiritualAlchemyTemplate)
}
