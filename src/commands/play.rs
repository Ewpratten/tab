use crate::{
    audio::join_guild_voice_channel,
    guild_utils::{check_user_has_sound_role, maybe_create_sound_role},
};
use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
};
use tracing::error;
use tracing::info;

#[command]
pub async fn play(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    info!("Executing play command from user: {}", msg.author);
    let guild = msg.guild(&ctx).await.unwrap();
    let member = msg.member.as_ref().unwrap();

    // Try to create the sound role
    maybe_create_sound_role(&ctx, &guild).await?;

    // Check the user permissions
    if check_user_has_sound_role(&guild, &member) {
        info!("User has sound role, executing play command");

        // Fetch the URL to play
        // TODO

        // Find where the member is in VC
        let member_vc = guild
            .voice_states
            .get(&msg.author.id)
            .and_then(|state| state.channel_id);

        // Join VC
        if let Some(channel_id) = member_vc {
            // Join
            join_guild_voice_channel(&ctx, guild.id, channel_id).await;

            let manager = songbird::get(ctx)
                .await
                .expect("Songbird Voice client placed in at initialisation.")
                .clone();

            if let Some(handler_lock) = manager.get(guild.id) {
                let mut handler = handler_lock.lock().await;
                // Get a source for "Mr Worldwide" entrance
                let source =
                    match songbird::ytdl("https://www.youtube.com/watch?v=FGFrTFakGJo").await {
                        Ok(source) => source,
                        Err(why) => {
                            error!("Err starting source: {:?}", why);
                            return Ok(());
                        }
                    };

                handler.play_source(source);
            }
        } else {
            msg.reply(&ctx.http, "You are not in a voice channel")
                .await?;
        }
    }
    Ok(())
}
