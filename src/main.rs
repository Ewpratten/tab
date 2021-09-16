use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg};

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
}
