use std::sync::Arc;

use serenity::prelude::{RwLock, TypeMapKey};

pub struct Song {
    pub url: url::Url,
}

pub struct SongQueue {
    pub songs: Vec<Song>,
}
impl Default for SongQueue {
    fn default() -> Self {
        Self {
            songs: Default::default(),
        }
    }
}

pub struct BotState;

impl TypeMapKey for BotState {
    type Value = Arc<RwLock<SongQueue>>;
}
