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
mod templates;

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
        .nest("/admin", admin::routes())
        .nest("/auth", auth::routes())
        .nest("/astrology", astrology::routes())
        .nest("/meditation", meditation::routes())
        .nest("/tarot", tarot::routes())
        .nest("/journal", journal::routes())
        .nest("/numerology", numerology::routes())
        .nest("/crystals", crystals::routes())
        .nest("/runes", runes::routes())
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
    HtmlTemplate(AppLandingTemplate { is_admin }).into_response()
}
