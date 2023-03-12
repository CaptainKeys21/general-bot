pub mod hooks;
pub mod ready;
pub mod all_shards_ready;
pub mod guild_create;
pub mod interaction_create;
pub mod checkers;

use serenity::model::prelude::Guild;
use serenity::{
    async_trait,
    model::{
        application::interaction::Interaction,
        gateway::Ready,
    },
    prelude::*,
    
};

// Event handler from serenity
pub struct Handler;



// Main event handler
#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {ready::ready(ctx, ready).await;}

    async fn guild_create(&self, ctx: Context, guild: Guild){guild_create::guild_create(ctx, guild).await;}

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {interaction_create::interaction_create(ctx, interaction).await;}
}





