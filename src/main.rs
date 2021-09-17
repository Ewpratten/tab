use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg};
use discord_utils::environment::DiscordEnvVars;

use crate::event_handler::init_bot_client;
use tracing::log::error;

mod audio;
mod commands;
mod event_handler;
mod guild_utils;
mod song_queue;

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
    let _guard = sentry::init((
        bot_env.sentry_dsn,
        sentry::ClientOptions {
            release: sentry::release_name!(),
            ..Default::default()
        },
    ));

    // Set up the bot client
    let mut client = init_bot_client(&bot_env.bot_token, &bot_env.bot_app_id)
        .await
        .expect("Failed to init bot client");

    // Run the client
    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}
