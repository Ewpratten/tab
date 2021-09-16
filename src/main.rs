use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg};

use crate::event_handler::init_bot_client;
use tracing::log::error;

mod commands;
mod event_handler;
mod song_queue;
mod guild_utils;
mod audio;

#[tokio::main]
async fn main() {
    let matches = App::new(crate_name!())
        .author(crate_authors!())
        .about(crate_description!())
        .version(crate_version!())
        .arg(
            Arg::with_name("dev_guild")
                .long("dev-guild")
                .takes_value(true)
                .help("Guild ID to use for development")
                .required(false),
        )
        .get_matches();

    // Enable logging
    tracing_subscriber::fmt::init();

    // Get data
    let dev_guild = matches.value_of("dev_guild");

    // Pull in data from the environment
    let discord_token = std::env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN not set");
    let discord_app_id = std::env::var("DISCORD_APP_ID")
        .expect("DISCORD_APP_ID not set")
        .parse()
        .expect("Application ID is not valid");
    // let sentry_

    // Set up the bot client
    let mut client = init_bot_client(&discord_token, &discord_app_id)
        .await
        .expect("Failed to init bot client");

    // Run the client
    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}
