use serenity::framework::standard::macros::group;
use ping::*;
mod ping;

#[group]
#[commands(ping)]
pub struct BotCommands;