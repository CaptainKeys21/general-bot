use serenity::{
    prelude::Context, 
    model::prelude::interaction::Interaction
};

use crate::{
    cache::{CommandCache, LoggerCache}, 
    services::logger::{
        CmdOrInt,
        LogType::*
    }
};



pub async fn interaction_create(ctx: Context, interaction: Interaction) {
    if let Interaction::ApplicationCommand(command) = interaction {
        let data_read = ctx.data.read().await;

        let log = data_read.get::<LoggerCache>();

        let command_manager = match data_read.get::<CommandCache>() {
            Some(m) => m.read().await,
            None => {
                if let Some(log) = log {
                    log.read().await.default(Error, "Command Manager not found");
                }
                return;
            }
        };

        if let Some(log) = log {
            log.read().await.command(Info, &command.data.name, CmdOrInt::Interaction(&command), None);
        }

        if let Err(why) = command_manager.on_command(&ctx, &command).await {
            if let Some(log) = log {
                log.read().await.default(Error, &format!("Command Manager error: {}", why));
            }
        }
    }
}