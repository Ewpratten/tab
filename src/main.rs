use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg};

mod commands;
mod song_queue;
mod event_handler;

#[tokio::main]
fn main() {
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

    // Get data
    let dev_guild = matches.value_of("dev_guild").unwrap();

    // Pull in data from the environment
    let token = std::env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN not set");
    let guild_id = std::env::var("GUILD_ID").expect("GUILD_ID not set");
    // let sentry_
}
