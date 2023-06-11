use serenity::{prelude::Context, model::prelude::{ChannelId, MessageId, GuildId}};

use crate::{cache::LoggerCache, services::logger::MsgUpdateLog};

pub async fn message_delete_bulk(ctx: Context, _channel_id: ChannelId, message_ids: Vec<MessageId>, _guild_id: Option<GuildId>) {
    let data = ctx.data.read().await;
    if let Some(log) = data.get::<LoggerCache>() {
        let logger = log.read().await;
        for msg_id in message_ids {
            logger.update_message_log(msg_id.0, MsgUpdateLog::Deleted)
        }
    };
}