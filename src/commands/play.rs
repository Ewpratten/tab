use crate::guild_utils::{check_user_has_sound_role, maybe_create_sound_role};
use discord_utils::{
    audio::controls::{join_guild_voice_channel, leave_guild_voice_channels, play_youtube_url},
    sentry_track_command,
    user::get_user_voice_channel,
};
use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
};
use tracing::error;
use tracing::info;
use url::Url;

#[command]
pub async fn play(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    sentry_track_command!(msg);
    info!("Executing play command from user: {}", msg.author);
    let guild = msg.guild(&ctx).await.unwrap();
    let member = msg.member.as_ref().unwrap();

    // Try to create the sound role
    maybe_create_sound_role(&ctx, &guild).await?;

    // Check the user permissions
    if check_user_has_sound_role(&guild, &member) {
        info!("User has sound role, executing play command");

        // Fetch the URL to play
        if let Ok(yt_url) = args.single::<String>() {
            // Find where the member is in VC
            let member_vc = get_user_voice_channel(&guild, &msg.author.id);

            // Join VC
            if let Some(channel_id) = member_vc {
                // Leave and Join
                leave_guild_voice_channels(&ctx, guild.id).await;
                std::thread::sleep(std::time::Duration::from_secs(3));
                join_guild_voice_channel(&ctx, guild.id, channel_id).await;

                let manager = songbird::get(ctx)
                    .await
                    .expect("Songbird Voice client placed in at initialization.")
                    .clone();

                if let Some(handler_lock) = manager.get(guild.id) {
                    let mut handler = handler_lock.lock().await;
                    // Get a source for "Mr Worldwide" entrance
                    play_youtube_url(
                        &Url::parse("https://www.youtube.com/watch?v=FGFrTFakGJo").unwrap(),
                        &mut handler,
                    )
                    .await
                    .expect("Failed to play youtube URL");

                    // Sleep through the video
                    std::thread::sleep(std::time::Duration::from_secs(3));

                    // Get the real video source
                    play_youtube_url(&Url::parse(&yt_url).unwrap(), &mut handler)
                        .await
                        .expect("Failed to play youtube URL");
                }
            } else {
                msg.reply(&ctx.http, "You are not in a voice channel")
                    .await?;
            }
        }
    }
    Ok(())
}
