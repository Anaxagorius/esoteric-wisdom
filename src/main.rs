mod admin;
mod akashic_records;
mod alchemy;
mod altered_states;
mod astrology;
mod auth;
mod ce5;
mod christianity;
mod conspiracy;
mod cryptozoology;
mod crystals;
mod druidism;
mod eastern_esotericism;
mod esoteric_concepts;
mod esoteric_corpora;
mod esoteric_figures;
mod esoteric_practices;
mod gateway_process;
mod hinduism;
mod indigenous_esotericism;
mod islam;
mod journal;
mod judaism;
mod khasarov_mirror;
mod meditation;
mod middle_eastern_esotericism;
mod mythologies_cosmologies;
mod nhi;
mod numerology;
mod orders_societies;
mod paranormal;
mod parapsychology;
mod remote_viewing;
mod runes;
mod shamanism;
mod state;
mod survival;
mod tarot;
mod templates;
mod uap;
mod western_esotericism;
mod wicca;
mod yoga;

use auth::HtmlTemplate;
use axum::{
    extract::State,
    http::{header, StatusCode},
    response::IntoResponse,
    routing::get,
    Router,
};
use state::AppState;
use templates::{AppLandingTemplate, LandingTemplate, OrganizationsTemplate, TimelineTemplate};
use tower_cookies::{CookieManagerLayer, Cookies};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let state = AppState::new().await?;

    let app = Router::new()
        .route("/", get(landing))
        .route("/home", get(app_landing))
        .route("/favicon.svg", get(favicon))
        .route("/og-image.png", get(og_image))
        .nest("/admin", admin::routes())
        .nest("/auth", auth::routes())
        .nest("/astrology", astrology::routes())
        .nest("/meditation", meditation::routes())
        .nest("/tarot", tarot::routes())
        .nest("/journal", journal::routes())
        .nest("/numerology", numerology::routes())
        .nest("/crystals", crystals::routes())
        .nest("/runes", runes::routes())
        .nest("/shamanism", shamanism::routes())
        .nest("/druidism", druidism::routes())
        .nest("/akashic-records", akashic_records::routes())
        .nest("/wicca", wicca::routes())
        .nest("/alchemy", alchemy::routes())
        .nest("/gateway-process", gateway_process::routes())
        .nest("/hinduism", hinduism::routes())
        .nest("/islam", islam::routes())
        .nest("/judaism", judaism::routes())
        .nest("/christianity", christianity::routes())
        .nest("/remote-viewing", remote_viewing::routes())
        .nest("/khasarov-mirror", khasarov_mirror::routes())
        .nest("/ce5", ce5::routes())
        .nest("/western-esotericism", western_esotericism::routes())
        .nest("/eastern-esotericism", eastern_esotericism::routes())
        .nest("/indigenous-esotericism", indigenous_esotericism::routes())
        .nest(
            "/middle-eastern-esotericism",
            middle_eastern_esotericism::routes(),
        )
        .nest("/esoteric-practices", esoteric_practices::routes())
        .nest("/esoteric-corpora", esoteric_corpora::routes())
        .nest("/orders-societies", orders_societies::routes())
        .nest(
            "/mythologies-cosmologies",
            mythologies_cosmologies::routes(),
        )
        .nest("/esoteric-figures", esoteric_figures::routes())
        .nest("/esoteric-concepts", esoteric_concepts::routes())
        .nest("/yoga", yoga::routes())
        .nest("/uap", uap::routes())
        .nest("/parapsychology", parapsychology::routes())
        .nest("/altered-states", altered_states::routes())
        .nest("/survival", survival::routes())
        .nest("/nhi", nhi::routes())
        .nest("/cryptozoology", cryptozoology::routes())
        .nest("/paranormal", paranormal::routes())
        .nest("/conspiracy", conspiracy::routes())
        .route("/timeline", get(timeline))
        .route("/organizations", get(organizations))
        .layer(CookieManagerLayer::new())
        .with_state(state);

    let port_env = std::env::var("PORT");
    let port: u16 = match &port_env {
        Ok(val) => val.parse().unwrap_or_else(|_| {
            tracing::warn!("Invalid PORT value '{val}', falling back to 3000");
            3000
        }),
        Err(_) => 3000,
    };
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], port));
    let url = format!("http://127.0.0.1:{port}");
    println!("✨ Esoteric Wisdom running at {url}");

    let listener = tokio::net::TcpListener::bind(addr).await?;
    if port_env.is_err() {
        if let Err(e) = open::that(&url) {
            tracing::warn!("Could not open browser automatically: {e}");
        }
    }

    axum::serve(listener, app).await?;

    Ok(())
}

async fn favicon() -> impl IntoResponse {
    (
        [(header::CONTENT_TYPE, "image/svg+xml")],
        include_bytes!("../static/favicon.svg").as_ref(),
    )
}

async fn og_image() -> impl IntoResponse {
    (
        [(header::CONTENT_TYPE, "image/png")],
        include_bytes!("../static/og-image.png").as_ref(),
    )
}

async fn landing() -> HtmlTemplate<LandingTemplate> {
    HtmlTemplate(LandingTemplate)
}

async fn timeline(State(state): State<AppState>) -> impl IntoResponse {
    let events = state.timeline.as_ref().clone();
    HtmlTemplate(TimelineTemplate { events })
}

async fn organizations(State(state): State<AppState>) -> impl IntoResponse {
    let organizations = state.organizations.as_ref().clone();
    HtmlTemplate(OrganizationsTemplate { organizations })
}

async fn app_landing(State(state): State<AppState>, cookies: Cookies) -> impl IntoResponse {
    let admin_claims = admin::get_admin_claims(&state, &cookies);
    if let Some(ref claims) = admin_claims {
        if claims.must_change_password {
            return (
                StatusCode::FOUND,
                [(header::LOCATION, "/admin/change-password")],
            )
                .into_response();
        }
    }
    let is_admin = admin_claims.is_some();
    let is_user = !is_admin
        && cookies
            .get("esoteric_session")
            .and_then(|c| auth::decode_token(&state, c.value()))
            .is_some();
    HtmlTemplate(AppLandingTemplate { is_admin, is_user }).into_response()
}
