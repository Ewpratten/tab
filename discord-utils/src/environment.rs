use serde::{Deserialize, Serialize};

/// Stores all relevant information about the environment the bot is launched in
#[derive(Debug, Deserialize, Serialize)]
pub struct DiscordEnvVars {
    /// Bot token used by Discord
    pub bot_token: String,
    /// App ID used by Discord
    pub bot_app_id: u64,
    /// Sentry DSN used optionally for tracking app status
    pub sentry_dsn: Option<String>,
}

impl DiscordEnvVars {
    /// Load env vars from the host
    pub fn load() -> Self {
        Self {
            bot_token: std::env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN not set"),
            bot_app_id: std::env::var("DISCORD_APP_ID")
                .expect("DISCORD_APP_ID not set")
                .parse()
                .expect("Application ID is not valid"),
            sentry_dsn: std::env::var("SENTRY_DSN").ok(),
        }
    }
}
