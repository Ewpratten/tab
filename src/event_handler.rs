use serenity::{async_trait, client::EventHandler, model::prelude::Activity};
use tracing::info;

pub struct BotEventHandler;

#[async_trait]
impl EventHandler for BotEventHandler {
    async fn ready(&self, ctx: serenity::client::Context, _ready: serenity::model::prelude::Ready) {
        info!("Bot is started");

        ctx.set_activity(Activity::playing("some tunes")).await;
    }
}
