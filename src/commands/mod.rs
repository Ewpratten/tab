use serenity::framework::standard::macros::group;
use ping::*;
use play::*;
use stop::*;
mod ping;
mod play;
mod stop;

#[group]
#[commands(ping, play, fuckoff)]
pub struct BotCommands;