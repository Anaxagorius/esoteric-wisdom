mod state;
mod auth;
mod admin;
mod astrology;
mod meditation;
mod tarot;
mod journal;
mod numerology;
mod crystals;
mod runes;
mod shamanism;
mod druidism;
mod akashic_records;
mod wicca;
mod alchemy;
mod gateway_process;
mod remote_viewing;
mod khasarov_mirror;
mod western_esotericism;
mod eastern_esotericism;
mod indigenous_esotericism;
mod middle_eastern_esotericism;
mod esoteric_practices;
mod orders_societies;
mod esoteric_corpora;
mod templates;
mod mythologies_cosmologies;

use axum::{Router, routing::get, extract::State, response::IntoResponse, http::{StatusCode, header}};
use tower_cookies::{CookieManagerLayer, Cookies};
use tracing_subscriber::EnvFilter;
use state::AppState;
use auth::HtmlTemplate;
use templates::{LandingTemplate, AppLandingTemplate};

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
        .nest("/remote-viewing", remote_viewing::routes())
        .nest("/khasarov-mirror", khasarov_mirror::routes())
        .nest("/western-esotericism", western_esotericism::routes())
        .nest("/eastern-esotericism", eastern_esotericism::routes())
        .nest("/indigenous-esotericism", indigenous_esotericism::routes())
        .nest("/middle-eastern-esotericism", middle_eastern_esotericism::routes())
        .nest("/esoteric-practices", esoteric_practices::routes())
        .nest("/esoteric-corpora", esoteric_corpora::routes())
        .nest("/orders-societies", orders_societies::routes())
        .nest("/mythologies-cosmologies", mythologies_cosmologies::routes())
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

async fn app_landing(
    State(state): State<AppState>,
    cookies: Cookies,
) -> impl IntoResponse {
    let admin_claims = admin::get_admin_claims(&state, &cookies);
    if let Some(ref claims) = admin_claims {
        if claims.must_change_password {
            return (StatusCode::FOUND, [(header::LOCATION, "/admin/change-password")]).into_response();
        }
    }
    let is_admin = admin_claims.is_some();
    let is_user = !is_admin && cookies
        .get("esoteric_session")
        .and_then(|c| auth::decode_token(&state, c.value()))
        .is_some();
    HtmlTemplate(AppLandingTemplate { is_admin, is_user }).into_response()
}
