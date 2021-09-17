use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg};
use discord_utils::{client::init_client, environment::DiscordEnvVars};

use crate::event_handler::BotEventHandler;
use tracing::log::error;

mod commands;
mod event_handler;
mod guild_utils;

#[tokio::main]
async fn main() {
    let matches = App::new(crate_name!())
        .author(crate_authors!())
        .about(crate_description!())
        .version(crate_version!())
        .get_matches();

    // Enable logging
    tracing_subscriber::fmt::init();

    // Pull in data from the environment
    let bot_env = DiscordEnvVars::load();
    if let Some(dsn) = &bot_env.sentry_dsn {
        let _guard = sentry::init((
            dsn.clone(),
            sentry::ClientOptions {
                release: sentry::release_name!(),
                ..Default::default()
            },
        ));
    }

    // Set up the bot client
    let mut client = init_client(
        &bot_env,
        vec![&commands::BOTCOMMANDS_GROUP],
        |c| c.allow_dm(true).prefix("!"),
        BotEventHandler,
    )
    .await
    .unwrap();

    // Run the client
    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}
