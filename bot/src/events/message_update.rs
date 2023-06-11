use poise::{FrameworkOptions, FrameworkContext, dispatch_event, Event};
use serenity::{
    prelude::Context,
    model::prelude::{Message, MessageUpdateEvent}, cache::CacheUpdate,
    Error
};

use crate::{cache::{LoggerCache, ShardManagerCache}, services::logger::{LogType, MsgUpdateLog}};



pub async fn message_update(ctx: Context, old: Option<Message>, new: Option<Message>, mut event: MessageUpdateEvent, options: &FrameworkOptions<(), Error>) {
    let data = ctx.data.read().await;
    if let Some(log) = data.get::<LoggerCache>() {
        let logger = log.read().await;
        match &new {
            Some(msg) => logger.message(LogType::Info, msg),
            None => {
                if let Some(msg) = event.update(&ctx.cache) {
                    logger.message(LogType::Info, &msg)
                }; 
            }
        };

        if let Some(msg) = &old {
            logger.update_message_log(msg.id.0, MsgUpdateLog::Edited)
        }
    };

    if let Some(shard_manager) = data.get::<ShardManagerCache>() {
        let framework_data = FrameworkContext {
            bot_id: ctx.cache.current_user_id(),
            user_data: &(),
            options,
            shard_manager
        };

        dispatch_event(framework_data, &ctx, &Event::MessageUpdate { old_if_available: old, new, event }).await;
    }
}