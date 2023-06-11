use poise::{FrameworkContext, FrameworkOptions, Event, dispatch_event};
use serenity::{
    prelude::Context, 
    model::prelude::interaction::Interaction,
    Error
};

use crate::{
    cache::{CommandCache, LoggerCache, ShardManagerCache}, 
    services::logger::{
        CmdOrInt,
        LogType::*
    }
};



pub async fn interaction_create(ctx: Context, interaction: Interaction, options: &FrameworkOptions<(), Error>) {
    let data = ctx.data.read().await;
    if let Interaction::ApplicationCommand(command) = &interaction {
        { // * Logger scope
            if let Some(log) = data.get::<LoggerCache>() {
                let logger = log.read().await;
                logger.command(Info, &command.data.name, CmdOrInt::Interaction(&command), None);
            };
        }

        // let command_manager = match data_read.get::<CommandCache>() {
        //     Some(m) => m.read().await,
        //     None => {
        //         if let Some(log) = log {
        //             log.read().await.default(Error, "Command Manager not found");
        //         }
        //         return;
        //     }
        // };

        // if let Some(log) = log {
        //     log.read().await.command(Info, &command.data.name, CmdOrInt::Interaction(&command), None);
        // }

        // if let Err(why) = command_manager.on_command(&ctx, &command).await {
        //     if let Some(log) = log {
        //         log.read().await.default(Error, &format!("Command Manager error: {}", why));
        //     }
        // }
    }

    if let Some(shard_manager) = data.get::<ShardManagerCache>() {
        let framework_data = FrameworkContext {
            bot_id: ctx.cache.current_user_id(),
            user_data: &(),
            options,
            shard_manager
        };

        dispatch_event(framework_data, &ctx, &Event::InteractionCreate { interaction }).await;
    }
}