pub mod hooks;
pub mod ready;
pub mod all_shards_ready;
pub mod guild_create;
pub mod interaction_create;
pub mod guild_member_addition;
pub mod guild_member_removal;
pub mod checkers;
pub mod message;

use serenity::model::prelude::{Guild, Member, GuildId, Message};
use serenity::model::user::User;
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

    async fn guild_create(&self, ctx: Context, guild: Guild, is_new: bool) {guild_create::guild_create(ctx, guild, is_new).await;}

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {interaction_create::interaction_create(ctx, interaction).await;}

    async fn guild_member_addition(&self, ctx: Context, new_member: Member) {guild_member_addition::guild_member_addition(ctx, new_member).await;}
    
    async fn guild_member_removal(&self, ctx: Context, guild_id: GuildId, user: User, member: Option<Member>) {guild_member_removal::guild_member_removal(ctx, guild_id, user, member).await;}

    async fn message(&self, ctx: Context, new_message: Message) {message::message(ctx, new_message).await;}
}





