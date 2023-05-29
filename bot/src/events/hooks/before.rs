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
    if let Some(log) = data.get::<LoggerCache>() {
        let logger = log.read().await;
        logger.command(Info, command_name, Command(msg), Some("START"));
    };

    true
}