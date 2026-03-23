use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::RwLock;
use tracing::info;
use uuid::Uuid;
use serde::{Serialize, Deserialize};

use crate::tarot::{TarotCard, DeckType, load_deck};
use crate::journal::JournalEntry;

#[derive(Clone)]
pub struct AppState {
    pub users: Arc<RwLock<Vec<User>>>,
    pub tarot_decks: Arc<RwLock<HashMap<String, Vec<TarotCard>>>>,
    pub journal_entries: Arc<RwLock<Vec<JournalEntry>>>,
    pub jwt_secret: Arc<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
}

impl AppState {
    pub async fn new() -> anyhow::Result<Self> {
        let mut decks: HashMap<String, Vec<TarotCard>> = HashMap::new();
        for deck_type in [DeckType::Rws, DeckType::Marseille, DeckType::Thoth] {
            let cards = load_deck(&deck_type).await?;
            decks.insert(deck_type.as_str().to_string(), cards);
        }

        let state = AppState {
            users: Arc::new(RwLock::new(Vec::new())),
            tarot_decks: Arc::new(RwLock::new(decks)),
            journal_entries: Arc::new(RwLock::new(Vec::new())),
            jwt_secret: Arc::new("change_me_super_secret".to_string()),
        };

        info!("AppState initialized with {} tarot decks", 3);
        Ok(state)
    }
}

impl User {
    pub fn new(email: String, password_hash: String) -> Self {
        User { id: Uuid::new_v4(), email, password_hash }
    }
}
