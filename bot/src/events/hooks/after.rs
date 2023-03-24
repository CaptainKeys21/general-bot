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
    let log = data.get::<LoggerCache>().unwrap().read().await;
    log.command(Info, command_name, Command(msg), Some("END"));

}