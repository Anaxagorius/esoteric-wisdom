use serde::{Serialize, Deserialize};
use std::fs;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TarotCard {
    pub id: String,
    pub name: String,
    pub arcana: String,
    pub suit: Option<String>,
    pub image_url: String,
    pub meaning_upright: String,
    pub meaning_reversed: String,
}

pub async fn load_tarot_deck() -> anyhow::Result<Vec<TarotCard>> {
    let data = fs::read_to_string("data/tarot_cards.json")?;
    let deck: Vec<TarotCard> = serde_json::from_str(&data)?;
    Ok(deck)
}

use axum::{routing::get, Router, extract::State, response::IntoResponse, http::StatusCode};
use rand::seq::SliceRandom;
use crate::state::AppState;
use crate::auth::HtmlTemplate;
use crate::templates::TarotDrawTemplate;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(tarot_home))
        .route("/draw", get(draw_card))
}

async fn tarot_home() -> impl IntoResponse {
    (StatusCode::FOUND, [("Location", "/tarot/draw")])
}

async fn draw_card(State(state): State<AppState>) -> impl IntoResponse {
    let mut deck = state.tarot_deck.write().await;
    let mut rng = rand::thread_rng();
    deck.shuffle(&mut rng);
    let card = deck.first().cloned();

    if let Some(card) = card {
        let reversed = rand::random::<bool>();
        let tpl = TarotDrawTemplate { card: &card, reversed };
        HtmlTemplate(tpl).into_response()
    } else {
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }
}
