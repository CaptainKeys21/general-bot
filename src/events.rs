use serenity::{
    async_trait,
    framework::{standard::macros::hook, standard::CommandResult, standard::DispatchError},
    model::{
        channel::Message, channel::ReactionType, event::MessageUpdateEvent, gateway::Ready,
        guild::Guild, id::ChannelId, id::GuildId, id::MessageId, interactions::Interaction,
        prelude::UnavailableGuild,
    },
    prelude::*,
    
};
use std::env;

use tokio::sync::MutexGuard;

use chrono::{DateTime, Utc};

use crate::{
    cache::*, commands,
};

pub struct Handler;

#[async_trait]
trait ShardsReadyHandler {
    async fn all_shards_ready(&self, ctx: &Context );
}

#[async_trait]
impl ShardsReadyHandler for Handler {
    async fn all_shards_ready(&self, ctx: &Context ) {
        let data = ctx.data.read().await;

        // register commands globally in release

        let mut cmd_mgr = data.get::<CommandCache>().unwrap().write().await;
        cmd_mgr.register_commands_global(ctx).await;
       
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, _ready: Ready) {
        println!("[Shard {}] Pronto", ctx.shard_id);       
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            let data_read = ctx.data.read().await;
            let commands = data_read.get::<CommandCache>().unwrap().read().await;
            match commands.on_command(&ctx, &command).await {
                Ok(_) => {}
                Err(e) => {
                    println!("Erro ao executar interação: {}", e);
                }
            }
        }
    }
}