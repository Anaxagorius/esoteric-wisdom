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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum DeckType {
    #[default]
    Rws,
    Marseille,
    Thoth,
    ModernWitch,
    LightSeers,
    EverydayTarot,
    MysticMondays,
    GoodTarot,
    MorganGreer,
    RealTalk,
    WildUnknown,
    Shadowscapes,
    Aquarian,
    DeviantMoon,
    AnnaK,
}

impl DeckType {
    pub fn from_str(s: &str) -> Self {
        match s {
            "marseille" => DeckType::Marseille,
            "thoth" => DeckType::Thoth,
            "modern_witch" => DeckType::ModernWitch,
            "light_seers" => DeckType::LightSeers,
            "everyday_tarot" => DeckType::EverydayTarot,
            "mystic_mondays" => DeckType::MysticMondays,
            "good_tarot" => DeckType::GoodTarot,
            "morgan_greer" => DeckType::MorganGreer,
            "real_talk" => DeckType::RealTalk,
            "wild_unknown" => DeckType::WildUnknown,
            "shadowscapes" => DeckType::Shadowscapes,
            "aquarian" => DeckType::Aquarian,
            "deviant_moon" => DeckType::DeviantMoon,
            "anna_k" => DeckType::AnnaK,
            _ => DeckType::Rws,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            DeckType::Rws => "rws",
            DeckType::Marseille => "marseille",
            DeckType::Thoth => "thoth",
            DeckType::ModernWitch => "modern_witch",
            DeckType::LightSeers => "light_seers",
            DeckType::EverydayTarot => "everyday_tarot",
            DeckType::MysticMondays => "mystic_mondays",
            DeckType::GoodTarot => "good_tarot",
            DeckType::MorganGreer => "morgan_greer",
            DeckType::RealTalk => "real_talk",
            DeckType::WildUnknown => "wild_unknown",
            DeckType::Shadowscapes => "shadowscapes",
            DeckType::Aquarian => "aquarian",
            DeckType::DeviantMoon => "deviant_moon",
            DeckType::AnnaK => "anna_k",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            DeckType::Rws => "Rider-Waite-Smith",
            DeckType::Marseille => "Tarot de Marseille",
            DeckType::Thoth => "Thoth Tarot",
            DeckType::ModernWitch => "Modern Witch Tarot",
            DeckType::LightSeers => "Light Seer's Tarot",
            DeckType::EverydayTarot => "Everyday Tarot",
            DeckType::MysticMondays => "Mystic Mondays Tarot",
            DeckType::GoodTarot => "The Good Tarot",
            DeckType::MorganGreer => "Morgan-Greer Tarot",
            DeckType::RealTalk => "Real Talk Tarot",
            DeckType::WildUnknown => "The Wild Unknown Tarot",
            DeckType::Shadowscapes => "Shadowscapes Tarot",
            DeckType::Aquarian => "Aquarian Tarot",
            DeckType::DeviantMoon => "Deviant Moon Tarot",
            DeckType::AnnaK => "Anna K Tarot",
        }
    }

    pub fn data_file(&self) -> &'static str {
        match self {
            DeckType::Rws => "data/tarot_cards_rws.json",
            DeckType::Marseille => "data/tarot_cards_marseille.json",
            DeckType::Thoth => "data/tarot_cards_thoth.json",
            DeckType::ModernWitch => "data/tarot_cards_modern_witch.json",
            DeckType::LightSeers => "data/tarot_cards_light_seers.json",
            DeckType::EverydayTarot => "data/tarot_cards_everyday_tarot.json",
            DeckType::MysticMondays => "data/tarot_cards_mystic_mondays.json",
            DeckType::GoodTarot => "data/tarot_cards_good_tarot.json",
            DeckType::MorganGreer => "data/tarot_cards_morgan_greer.json",
            DeckType::RealTalk => "data/tarot_cards_real_talk.json",
            DeckType::WildUnknown => "data/tarot_cards_wild_unknown.json",
            DeckType::Shadowscapes => "data/tarot_cards_shadowscapes.json",
            DeckType::Aquarian => "data/tarot_cards_aquarian.json",
            DeckType::DeviantMoon => "data/tarot_cards_deviant_moon.json",
            DeckType::AnnaK => "data/tarot_cards_anna_k.json",
        }
    }
}

pub async fn load_deck(deck_type: &DeckType) -> anyhow::Result<Vec<TarotCard>> {
    let data = fs::read_to_string(deck_type.data_file())?;
    let deck: Vec<TarotCard> = serde_json::from_str(&data)?;
    Ok(deck)
}

use axum::{routing::get, Router, extract::{State, Query}, response::IntoResponse, http::StatusCode};
use rand::seq::SliceRandom;
use crate::state::AppState;
use crate::templates::HtmlTemplate;
use crate::templates::{TarotDrawTemplate, TarotSelectTemplate};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(tarot_home))
        .route("/select", get(tarot_select))
        .route("/draw", get(draw_card))
}

async fn tarot_home() -> impl IntoResponse {
    (StatusCode::FOUND, [("Location", "/tarot/select")])
}

async fn tarot_select() -> impl IntoResponse {
    HtmlTemplate(TarotSelectTemplate).into_response()
}

#[derive(Deserialize)]
struct DrawQuery {
    deck: Option<String>,
}

async fn draw_card(
    State(state): State<AppState>,
    Query(query): Query<DrawQuery>,
) -> impl IntoResponse {
    let deck_type = query.deck
        .as_deref()
        .map(DeckType::from_str)
        .unwrap_or_default();

    let decks = state.tarot_decks.read().await;
    let deck = decks.get(&deck_type.as_str().to_string());

    if let Some(cards) = deck {
        let mut cards = cards.clone();
        let mut rng = rand::thread_rng();
        cards.shuffle(&mut rng);
        if let Some(card) = cards.first().cloned() {
            let reversed = rand::random::<bool>();
            let tpl = TarotDrawTemplate {
                card: &card,
                reversed,
                deck_name: deck_type.display_name(),
                deck_key: deck_type.as_str(),
            };
            return HtmlTemplate(tpl).into_response();
        }
    }

    StatusCode::INTERNAL_SERVER_ERROR.into_response()
}
