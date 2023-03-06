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

        let commands = data_read.get::<CommandCache>().unwrap().read().await;
        let log = data_read.get::<LoggerCache>().unwrap().read().await;

        log.command(Info, &command.data.name, CmdOrInt::Interaction(&command), None).await;

        match commands.on_command(&ctx, &command).await {
            Ok(_) => {}
            Err(e) => {
                println!("Erro ao executar interação: {}", e);
            }
        }
    }
}