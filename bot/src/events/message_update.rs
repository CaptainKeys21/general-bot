use serenity::{
    prelude::Context,
    model::prelude::{Message, MessageUpdateEvent}, cache::CacheUpdate,
};

use crate::{cache::LoggerCache, services::logger::{LogType, MsgUpdateLog}};



pub async fn message_update(ctx: Context, old: Option<Message>, new: Option<Message>, mut event: MessageUpdateEvent) {
    let data = ctx.data.read().await;
    if let Some(log) = data.get::<LoggerCache>() {
        let logger = log.read().await;
        match new {
            Some(msg) => logger.message(LogType::Info, &msg),
            None => {
                if let Some(msg) = event.update(&ctx.cache) {
                    logger.message(LogType::Info, &msg)
                }; 
            }
        };

        if let Some(msg) = old {
            logger.update_message_log(msg.id.0, MsgUpdateLog::Edited)
        }
    };
}