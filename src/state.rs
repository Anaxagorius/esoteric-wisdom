use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::RwLock;
use tracing::info;
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use argon2::{Argon2, PasswordHasher};
use argon2::password_hash::SaltString;

use crate::tarot::{TarotCard, DeckType, load_deck};
use crate::journal::JournalEntry;

pub const ADMIN_USERNAME: &str = "AngieMaidment#1";
// Initial credential — admin is prompted to set a new password on first login
const ADMIN_INITIAL_PASSWORD: &str = "Loveadored69$";

#[derive(Clone)]
pub struct AppState {
    pub users: Arc<RwLock<Vec<User>>>,
    pub tarot_decks: Arc<RwLock<HashMap<String, Vec<TarotCard>>>>,
    pub journal_entries: Arc<RwLock<Vec<JournalEntry>>>,
    pub jwt_secret: Arc<String>,
    pub admin_password_hash: Arc<RwLock<String>>,
    pub admin_must_change_password: Arc<RwLock<bool>>,
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
        let all_decks = [
            DeckType::Rws,
            DeckType::Marseille,
            DeckType::Thoth,
            DeckType::ModernWitch,
            DeckType::LightSeers,
            DeckType::EverydayTarot,
            DeckType::MysticMondays,
            DeckType::GoodTarot,
            DeckType::MorganGreer,
            DeckType::RealTalk,
            DeckType::WildUnknown,
            DeckType::Shadowscapes,
            DeckType::Aquarian,
            DeckType::DeviantMoon,
            DeckType::AnnaK,
        ];
        let deck_count = all_decks.len();
        for deck_type in all_decks {
            let cards = load_deck(&deck_type).await?;
            decks.insert(deck_type.as_str().to_string(), cards);
        }

        let salt = SaltString::generate(&mut rand::thread_rng());
        let argon2 = Argon2::default();
        let admin_hash = argon2
            .hash_password(ADMIN_INITIAL_PASSWORD.as_bytes(), &salt)
            .map_err(|e| anyhow::anyhow!("Failed to hash admin password: {e}"))?
            .to_string();

        let state = AppState {
            users: Arc::new(RwLock::new(Vec::new())),
            tarot_decks: Arc::new(RwLock::new(decks)),
            journal_entries: Arc::new(RwLock::new(Vec::new())),
            jwt_secret: Arc::new("change_me_super_secret".to_string()),
            admin_password_hash: Arc::new(RwLock::new(admin_hash)),
            admin_must_change_password: Arc::new(RwLock::new(true)),
        };

        info!("AppState initialized with {} tarot decks", deck_count);
        Ok(state)
    }
}

impl User {
    pub fn new(email: String, password_hash: String) -> Self {
        User { id: Uuid::new_v4(), email, password_hash }
    }
}
