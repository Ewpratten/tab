use serenity::framework::standard::macros::group;
use ping::*;
use play::*;
mod ping;
mod play;

#[group]
#[commands(ping, play)]
pub struct BotCommands;