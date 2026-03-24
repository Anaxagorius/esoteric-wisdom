mod state;
mod auth;
mod astrology;
mod meditation;
mod tarot;
mod journal;
mod numerology;
mod templates;

use axum::{Router, routing::get};
use tower_cookies::CookieManagerLayer;
use tracing_subscriber::EnvFilter;
use state::AppState;
use auth::HtmlTemplate;
use templates::LandingTemplate;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let state = AppState::new().await?;

    let app = Router::new()
        .route("/", get(landing))
        .nest("/auth", auth::routes())
        .nest("/astrology", astrology::routes())
        .nest("/meditation", meditation::routes())
        .nest("/tarot", tarot::routes())
        .nest("/journal", journal::routes())
        .nest("/numerology", numerology::routes())
        .layer(CookieManagerLayer::new())
        .with_state(state);

    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
    let url = format!("http://{addr}");
    println!("✨ Esoteric Wisdom running at {url}");

    let listener = tokio::net::TcpListener::bind(addr).await?;
    if let Err(e) = open::that(&url) {
        tracing::warn!("Could not open browser automatically: {e}");
    }

    axum::serve(listener, app).await?;

    Ok(())
}

async fn landing() -> HtmlTemplate<LandingTemplate> {
    HtmlTemplate(LandingTemplate)
}
