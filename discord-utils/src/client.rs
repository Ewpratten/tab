use serenity::{
    client::EventHandler,
    framework::{
        standard::{CommandGroup, Configuration},
        StandardFramework,
    },
    Client,
};
use songbird::{SerenityInit, Songbird};

use crate::environment::DiscordEnvVars;

pub async fn init_client<Handler, Callback>(
    env: &DiscordEnvVars,
    command_groups: Vec<&'static CommandGroup>,
    config_callback: Callback,
    handler: Handler,
) -> Result<Client, serenity::Error>
where
    Handler: EventHandler + 'static,
    Callback: FnOnce(&mut Configuration) -> &mut Configuration,
{
    // Set up the framework
    let mut framework = StandardFramework::new().configure(config_callback);

    // Set up all command groups
    for command_group in command_groups {
        framework = framework.group(command_group);
    }

    // Get a voice context
    let voice = Songbird::serenity();

    // Set up the client
    let client = Client::builder(&env.bot_token)
        .application_id(env.bot_app_id)
        .framework(framework)
        .event_handler(handler)
        .register_songbird_with(voice)
        .await?;

    Ok(client)
}
