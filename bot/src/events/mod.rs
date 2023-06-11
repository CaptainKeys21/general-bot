pub mod hooks;
pub mod ready;
pub mod all_shards_ready;
pub mod guild_create;
pub mod interaction_create;
pub mod guild_member_addition;
pub mod guild_member_removal;
pub mod checkers;
pub mod message;
pub mod message_update;
pub mod message_delete;
pub mod message_delete_bulk;

use serenity::model::prelude::{Guild, Member, GuildId, Message, MessageUpdateEvent, ChannelId, MessageId};
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

    async fn message_update(&self, ctx: Context, old: Option<Message>, new: Option<Message>, event: MessageUpdateEvent) {message_update::message_update(ctx, old, new, event).await;}
    
    async fn message_delete(&self, ctx: Context, channel_id: ChannelId, deleted_message_id: MessageId, guild_id: Option<GuildId>) {message_delete::message_delete(ctx, channel_id, deleted_message_id, guild_id).await;}

    async fn message_delete_bulk(&self, ctx: Context, channel_id: ChannelId, deleted_message_ids: Vec<MessageId>, guild_id: Option<GuildId>) {message_delete_bulk::message_delete_bulk(ctx, channel_id, deleted_message_ids, guild_id).await;}
}





