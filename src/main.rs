mod state;
mod auth;
mod astrology;
mod meditation;
mod tarot;
mod journal;
mod templates;

use axum::{Router, routing::get};
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
        .with_state(state);

    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("✨ Esoteric Wisdom running at http://{addr}");

    axum::serve(tokio::net::TcpListener::bind(addr).await?, app).await?;

    Ok(())
}

async fn landing() -> HtmlTemplate<LandingTemplate> {
    HtmlTemplate(LandingTemplate)
}
