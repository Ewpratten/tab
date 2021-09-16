use std::sync::Arc;

use serenity::prelude::{RwLock, TypeMapKey};
use songbird::input::Input;
use tracing::error;

pub struct Song {
    pub url: url::Url,
}

pub struct SongQueue {
    pub songs: Vec<Song>,
}

impl SongQueue {
    /// Construct a new SongQueue.
    pub async fn new() -> Self {
        Self {
            songs: Default::default(),
        }
    }
}

pub struct BotState;

impl TypeMapKey for BotState {
    type Value = Arc<RwLock<SongQueue>>;
}
