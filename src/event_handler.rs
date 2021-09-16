use std::sync::Arc;

use serenity::{
    async_trait, client::EventHandler, framework::StandardFramework, model::prelude::Activity,
    prelude::RwLock, Client,
};
use songbird::{SerenityInit, Songbird};
use tracing::info;

use crate::{commands, song_queue::{BotState, SongQueue}};

pub struct BotEventHandler;

#[async_trait]
impl EventHandler for BotEventHandler {
    async fn ready(&self, ctx: serenity::client::Context, _ready: serenity::model::prelude::Ready) {
        info!("Bot is started");

        ctx.set_activity(Activity::playing("some tunes")).await;
    }
}

/// Set up and create the bot client
pub async fn init_bot_client(
    discord_token: &str,
    discord_app_id: &u64,
) -> Result<Client, serenity::Error> {
    // Set up the framework
    let framework = StandardFramework::new()
        .configure(|c| c.allow_dm(true).prefix("!"))
        .group(&commands::BOTCOMMANDS_GROUP);

    // Get a voice context
    let voice = Songbird::serenity();

    // Set up the client
    let client = Client::builder(discord_token)
        .application_id(*discord_app_id)
        .framework(framework)
        .event_handler(BotEventHandler)
        .register_songbird_with(voice)
        .await?;

    // Set up the song queue to be accessible between command calls
    {
        let mut data = client.data.write().await;
        data.insert::<BotState>(Arc::new(RwLock::new(SongQueue::default())));
    }

    Ok(client)
}
