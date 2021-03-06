use crate::guild_utils::{check_user_has_sound_role, maybe_create_sound_role};
use discord_utils::{audio::controls::leave_guild_voice_channels, sentry_track_command};
use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
};
use tracing::error;
use tracing::info;

#[command]
pub async fn fuckoff(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    sentry_track_command!(msg);
    info!("Executing fuckoff command from user: {}", msg.author);
    let guild = msg.guild(&ctx).await.unwrap();
    let member = msg.member.as_ref().unwrap();

    // Try to create the sound role
    maybe_create_sound_role(&ctx, &guild).await?;

    // Check the user permissions
    if check_user_has_sound_role(&guild, &member) {
        info!("User has sound role, executing fuckoff command");

        leave_guild_voice_channels(&ctx, guild.id).await;
    }
    Ok(())
}
