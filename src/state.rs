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

pub fn journal_data_path() -> String {
    std::env::var("JOURNAL_DATA_FILE").unwrap_or_else(|_| "data/journal_entries.json".to_string())
}

pub fn users_data_path() -> String {
    std::env::var("USERS_DATA_FILE").unwrap_or_else(|_| "data/users.json".to_string())
}

pub fn admin_data_path() -> String {
    std::env::var("ADMIN_DATA_FILE").unwrap_or_else(|_| "data/admin.json".to_string())
}

async fn load_from_file<T: serde::de::DeserializeOwned + Default>(path: &str) -> T {
    match tokio::fs::read_to_string(path).await {
        Ok(contents) => serde_json::from_str(&contents).unwrap_or_else(|e| {
            tracing::warn!("Failed to parse data from {path}: {e}");
            T::default()
        }),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => T::default(),
        Err(e) => {
            tracing::warn!("Could not read data from {path}: {e}");
            T::default()
        }
    }
}

async fn save_to_file<T: serde::Serialize + ?Sized>(path: &str, value: &T) {
    match serde_json::to_string(value) {
        Ok(json) => {
            if let Some(parent) = std::path::Path::new(path).parent() {
                let _ = tokio::fs::create_dir_all(parent).await;
            }
            if let Err(e) = tokio::fs::write(path, json).await {
                tracing::warn!("Failed to save data to {path}: {e}");
            }
        }
        Err(e) => tracing::warn!("Failed to serialize data for {path}: {e}"),
    }
}

async fn load_journal_entries() -> Vec<JournalEntry> {
    load_from_file::<Vec<JournalEntry>>(&journal_data_path()).await
}

async fn load_users() -> Vec<User> {
    load_from_file::<Vec<User>>(&users_data_path()).await
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AdminData {
    pub password_hash: String,
    pub must_change_password: bool,
}

impl Default for AdminData {
    fn default() -> Self {
        AdminData {
            password_hash: String::new(),
            must_change_password: true,
        }
    }
}

pub async fn save_users(users: &[User]) {
    save_to_file(&users_data_path(), users).await;
}

pub async fn save_admin_data(data: &AdminData) {
    save_to_file(&admin_data_path(), data).await;
}

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

        let journal_entries = load_journal_entries().await;
        let journal_count = journal_entries.len();

        let users = load_users().await;
        let user_count = users.len();

        // Load persisted admin data; generate initial hash only when no saved data exists
        let admin_data = {
            let saved: AdminData = load_from_file(&admin_data_path()).await;
            if saved.password_hash.is_empty() {
                let salt = SaltString::generate(&mut rand::thread_rng());
                let argon2 = Argon2::default();
                let hash = argon2
                    .hash_password(ADMIN_INITIAL_PASSWORD.as_bytes(), &salt)
                    .map_err(|e| anyhow::anyhow!("Failed to hash admin password: {e}"))?
                    .to_string();
                AdminData { password_hash: hash, must_change_password: true }
            } else {
                saved
            }
        };

        let jwt_secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| {
            tracing::warn!("JWT_SECRET env var not set — using insecure default. Set JWT_SECRET in production!");
            "change_me_super_secret".to_string()
        });

        let state = AppState {
            users: Arc::new(RwLock::new(users)),
            tarot_decks: Arc::new(RwLock::new(decks)),
            journal_entries: Arc::new(RwLock::new(journal_entries)),
            jwt_secret: Arc::new(jwt_secret),
            admin_password_hash: Arc::new(RwLock::new(admin_data.password_hash)),
            admin_must_change_password: Arc::new(RwLock::new(admin_data.must_change_password)),
        };

        info!(
            "AppState initialized with {} tarot decks, {} journal entries, {} users",
            deck_count, journal_count, user_count
        );
        Ok(state)
    }
}

impl User {
    pub fn new(email: String, password_hash: String) -> Self {
        User { id: Uuid::new_v4(), email, password_hash }
    }
}
