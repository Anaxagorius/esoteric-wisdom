use axum::{routing::get, Router, response::IntoResponse};
use crate::auth::HtmlTemplate;
use crate::templates::{
    EsotericMythologiesCosmologiesTemplate,
    GnosticAeonsTemplate,
    DemiurgeMythTemplate,
    SophiaTraditionsTemplate,
    TreeOfLifeCosmologyTemplate,
    EmanationTheoriesTemplate,
    PlatonicWorldSoulTemplate,
    AstralPlanesTemplate,
    ChakricSystemsTemplate,
    SephirothicHierarchiesTemplate,
};
use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(hub))
        .route("/gnostic-aeons", get(gnostic_aeons))
        .route("/demiurge-myth", get(demiurge_myth))
        .route("/sophia-traditions", get(sophia_traditions))
        .route("/tree-of-life-cosmology", get(tree_of_life_cosmology))
        .route("/emanation-theories", get(emanation_theories))
        .route("/platonic-world-soul", get(platonic_world_soul))
        .route("/astral-planes", get(astral_planes))
        .route("/chakric-systems", get(chakric_systems))
        .route("/sephirothic-hierarchies", get(sephirothic_hierarchies))
}

async fn hub() -> impl IntoResponse {
    HtmlTemplate(EsotericMythologiesCosmologiesTemplate)
}

async fn gnostic_aeons() -> impl IntoResponse {
    HtmlTemplate(GnosticAeonsTemplate)
}

async fn demiurge_myth() -> impl IntoResponse {
    HtmlTemplate(DemiurgeMythTemplate)
}

async fn sophia_traditions() -> impl IntoResponse {
    HtmlTemplate(SophiaTraditionsTemplate)
}

async fn tree_of_life_cosmology() -> impl IntoResponse {
    HtmlTemplate(TreeOfLifeCosmologyTemplate)
}

async fn emanation_theories() -> impl IntoResponse {
    HtmlTemplate(EmanationTheoriesTemplate)
}

async fn platonic_world_soul() -> impl IntoResponse {
    HtmlTemplate(PlatonicWorldSoulTemplate)
}

async fn astral_planes() -> impl IntoResponse {
    HtmlTemplate(AstralPlanesTemplate)
}

async fn chakric_systems() -> impl IntoResponse {
    HtmlTemplate(ChakricSystemsTemplate)
}

async fn sephirothic_hierarchies() -> impl IntoResponse {
    HtmlTemplate(SephirothicHierarchiesTemplate)
}
