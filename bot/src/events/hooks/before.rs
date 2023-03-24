use serenity::{
    framework::standard::macros::hook,
    prelude::Context,
    model::channel::Message
};
use crate::{
    services::logger::{
        CmdOrInt::Command,
        LogType::*
    },
    cache::LoggerCache,
};

// Command hook, runs before every command
#[hook]
pub async fn before(ctx: &Context, msg: &Message, command_name: &str) -> bool {
    let data = ctx.data.read().await;
    let log = data.get::<LoggerCache>().unwrap().read().await;
    log.command(Info, command_name, Command(msg), Some("START"));

    true
}