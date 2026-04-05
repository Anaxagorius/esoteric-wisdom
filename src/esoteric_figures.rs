use axum::{routing::get, Router, response::IntoResponse};
use crate::auth::HtmlTemplate;
use crate::templates::{
    EsotericFiguresTemplate,
    HermesTrismegistusTemplate,
    ThothTemplate,
    EnochMetatronTemplate,
    OrpheusFigureTemplate,
    MelchizedekTemplate,
    ZoroasterFigureTemplate,
    PythagorasFigureTemplate,
    PlatinusTemplate,
    IamblicusTemplate,
    ParacelsusTemplate,
    MarsilisFicinoTemplate,
    JacobBohmeTemplate,
    EmanuelSwedenborgTemplate,
    HelenaBlavatskyTemplate,
    RudolfSteinerTemplate,
    DionFortuneTemplate,
    EliphasLeviTemplate,
    PapusTemplate,
};
use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(hub))
        .route("/hermes-trismegistus", get(hermes_trismegistus))
        .route("/thoth", get(thoth))
        .route("/enoch-metatron", get(enoch_metatron))
        .route("/orpheus", get(orpheus))
        .route("/melchizedek", get(melchizedek))
        .route("/zoroaster", get(zoroaster))
        .route("/pythagoras", get(pythagoras))
        .route("/plotinus", get(plotinus))
        .route("/iamblichus", get(iamblichus))
        .route("/paracelsus", get(paracelsus))
        .route("/marsilio-ficino", get(marsilio_ficino))
        .route("/jacob-boehme", get(jacob_boehme))
        .route("/emanuel-swedenborg", get(emanuel_swedenborg))
        .route("/helena-blavatsky", get(helena_blavatsky))
        .route("/rudolf-steiner", get(rudolf_steiner))
        .route("/dion-fortune", get(dion_fortune))
        .route("/eliphas-levi", get(eliphas_levi))
        .route("/papus", get(papus))
}

async fn hub() -> impl IntoResponse { HtmlTemplate(EsotericFiguresTemplate) }
async fn hermes_trismegistus() -> impl IntoResponse { HtmlTemplate(HermesTrismegistusTemplate) }
async fn thoth() -> impl IntoResponse { HtmlTemplate(ThothTemplate) }
async fn enoch_metatron() -> impl IntoResponse { HtmlTemplate(EnochMetatronTemplate) }
async fn orpheus() -> impl IntoResponse { HtmlTemplate(OrpheusFigureTemplate) }
async fn melchizedek() -> impl IntoResponse { HtmlTemplate(MelchizedekTemplate) }
async fn zoroaster() -> impl IntoResponse { HtmlTemplate(ZoroasterFigureTemplate) }
async fn pythagoras() -> impl IntoResponse { HtmlTemplate(PythagorasFigureTemplate) }
async fn plotinus() -> impl IntoResponse { HtmlTemplate(PlatinusTemplate) }
async fn iamblichus() -> impl IntoResponse { HtmlTemplate(IamblicusTemplate) }
async fn paracelsus() -> impl IntoResponse { HtmlTemplate(ParacelsusTemplate) }
async fn marsilio_ficino() -> impl IntoResponse { HtmlTemplate(MarsilisFicinoTemplate) }
async fn jacob_boehme() -> impl IntoResponse { HtmlTemplate(JacobBohmeTemplate) }
async fn emanuel_swedenborg() -> impl IntoResponse { HtmlTemplate(EmanuelSwedenborgTemplate) }
async fn helena_blavatsky() -> impl IntoResponse { HtmlTemplate(HelenaBlavatskyTemplate) }
async fn rudolf_steiner() -> impl IntoResponse { HtmlTemplate(RudolfSteinerTemplate) }
async fn dion_fortune() -> impl IntoResponse { HtmlTemplate(DionFortuneTemplate) }
async fn eliphas_levi() -> impl IntoResponse { HtmlTemplate(EliphasLeviTemplate) }
async fn papus() -> impl IntoResponse { HtmlTemplate(PapusTemplate) }
