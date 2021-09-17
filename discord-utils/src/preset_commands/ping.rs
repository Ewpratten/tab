use crate::sentry_track_command;
use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
};
use tracing::info;

#[command]
pub async fn ping(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    sentry_track_command!(msg);
    info!("Executing ping command from user: {}", msg.author);
    msg.channel_id.say(&ctx.http, "Pong!").await?;
    Ok(())
}
