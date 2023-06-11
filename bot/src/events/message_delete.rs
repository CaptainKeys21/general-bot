use serenity::{
    prelude::Context, 
    model::prelude::{
        ChannelId, 
        GuildId, 
        MessageId
    }
};

use crate::{cache::LoggerCache, services::logger::MsgUpdateLog};

pub async fn message_delete(ctx: Context, _channel_id: ChannelId, msg_id: MessageId, _guild_id: Option<GuildId>) {
    let data = ctx.data.read().await;
    if let Some(log) = data.get::<LoggerCache>() {
        let logger = log.read().await;
        logger.update_message_log(msg_id.0, MsgUpdateLog::Deleted)
    };
}