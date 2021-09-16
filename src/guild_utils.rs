use sentry::User;
use serenity::{
    client::Context,
    model::guild::{Guild, PartialMember},
};

const SOUND_ROLE_NAME: &str = "Sound Control";

pub fn check_user_has_sound_role(guild: &Guild, user: &PartialMember) -> bool {
    let guild_sound_role_id = guild
        .roles
        .iter()
        .filter(|(_id, role)| role.name == SOUND_ROLE_NAME)
        .next()
        .unwrap().0;
    user.roles.iter().any(|role| role == guild_sound_role_id)
}

pub async fn maybe_create_sound_role(ctx: &Context, guild: &Guild) -> Result<(), serenity::Error> {
    if !guild
        .roles
        .values()
        .any(|role| role.name == SOUND_ROLE_NAME)
    {
        // Create the sound role
        guild
            .create_role(&ctx.http, |r| r.name(SOUND_ROLE_NAME))
            .await?;
    }
    Ok(())
}
