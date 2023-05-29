use serenity::{
    framework::standard::{
        macros::hook,
        CommandResult,
    },
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


// Command hook, runs after every command
#[hook]
pub async fn after(ctx: &Context, msg: &Message, command_name: &str, _command_result: CommandResult) {
    let data = ctx.data.read().await;
    if let Some(log) = data.get::<LoggerCache>() {
        let logger = log.read().await;
        logger.command(Info, command_name, Command(msg), Some("END"));
    };
}