use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;
use uuid::Uuid;
use serde::{Serialize, Deserialize};

use crate::tarot::TarotCard;
use crate::journal::JournalEntry;

#[derive(Clone)]
pub struct AppState {
    pub users: Arc<RwLock<Vec<User>>>,
    pub tarot_deck: Arc<RwLock<Vec<TarotCard>>>,
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
        let tarot_deck = crate::tarot::load_tarot_deck().await?;

        let state = AppState {
            users: Arc::new(RwLock::new(Vec::new())),
            tarot_deck: Arc::new(RwLock::new(tarot_deck)),
            journal_entries: Arc::new(RwLock::new(Vec::new())),
            jwt_secret: Arc::new("change_me_super_secret".to_string()),
        };

        info!("AppState initialized");
        Ok(state)
    }
}

impl User {
    pub fn new(email: String, password_hash: String) -> Self {
        User { id: Uuid::new_v4(), email, password_hash }
    }
}
