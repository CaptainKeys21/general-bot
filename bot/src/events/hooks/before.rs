use poise::{
    Context, 
    BoxFuture, 
    ApplicationCommandOrAutocompleteInteraction::ApplicationCommand
};

use serenity::Error;

use crate::{
    services::logger::{
        CmdOrInt::Command,
        CmdOrInt::Interaction,
        LogType::*
    },
    cache::LoggerCache,
};

pub fn pre_command(ctx: Context<'_, (), Error>) -> BoxFuture<'_, ()> {
    Box::pin(async move {
        let data = ctx.serenity_context().data.read().await;
        let command_name = &ctx.command().name;
        if let Some(log) = data.get::<LoggerCache>() {
            let logger = log.read().await;
            match &ctx {
                Context::Prefix(ctx_pfx) => {
                    logger.command(Info, command_name, Command(ctx_pfx.msg), Some("START"));
                }
                Context::Application(ctx_app) => {
                    if let ApplicationCommand(int) = ctx_app.interaction {
                        logger.command(Info, command_name, Interaction(int), Some("START"));
                    }
                }
            }
        };
    })
}