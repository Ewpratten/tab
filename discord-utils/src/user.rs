use serenity::model::{
    guild::Guild,
    id::{ChannelId, UserId},
};

/// Gets a user's current voice channel ID
pub fn get_user_voice_channel(guild: &Guild, user_id: &UserId) -> Option<ChannelId> {
    guild
        .voice_states
        .get(user_id)
        .and_then(|state| state.channel_id)
}
