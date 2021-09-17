use serenity::framework::standard::macros::group;
use discord_utils::preset_commands::ping::*;
use play::*;
use stop::*;
mod play;
mod stop;

#[group]
#[commands(ping, play, fuckoff)]
pub struct BotCommands;