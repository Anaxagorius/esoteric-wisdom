use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;

use crate::journal::JournalEntry;
use crate::tarot::{load_deck, DeckType, TarotCard};

pub fn journal_data_path() -> String {
    std::env::var("JOURNAL_DATA_FILE").unwrap_or_else(|_| "data/journal_entries.json".to_string())
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

async fn load_journal_entries() -> Vec<JournalEntry> {
    load_from_file::<Vec<JournalEntry>>(&journal_data_path()).await
}

fn load_organizations() -> Vec<Organization> {
    serde_json::from_str(include_str!("../content/organizations.json")).unwrap_or_default()
}

fn load_timeline() -> Vec<TimelineEvent> {
    serde_json::from_str(include_str!("../content/timeline.json")).unwrap_or_default()
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Organization {
    pub name: String,
    pub category: String,
    pub description: String,
    pub website: Option<String>,
    pub founded: Option<String>,
    pub notes: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TimelineEvent {
    pub date: String,
    pub title: String,
    pub description: String,
    pub category: String,
    pub sources: Vec<String>,
    pub link: Option<String>,
    pub article_slug: Option<String>,
}

#[derive(Clone)]
pub struct AppState {
    pub tarot_decks: Arc<RwLock<HashMap<String, Vec<TarotCard>>>>,
    pub journal_entries: Arc<RwLock<Vec<JournalEntry>>>,
    pub organizations: Arc<Vec<Organization>>,
    pub timeline: Arc<Vec<TimelineEvent>>,
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

        let organizations = load_organizations();
        let organization_count = organizations.len();

        let timeline = load_timeline();
        let timeline_count = timeline.len();

        let state = AppState {
            tarot_decks: Arc::new(RwLock::new(decks)),
            journal_entries: Arc::new(RwLock::new(journal_entries)),
            organizations: Arc::new(organizations),
            timeline: Arc::new(timeline),
        };

        info!(
            "AppState initialized with {} tarot decks, {} journal entries, {} organizations, {} timeline events",
            deck_count, journal_count, organization_count, timeline_count
        );
        Ok(state)
    }
}
