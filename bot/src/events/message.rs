use serenity::{
    prelude::Context, 
    model::prelude::Message, 
    Error
};
use poise::{
    FrameworkContext,
    FrameworkOptions,
    dispatch_event,
    Event
};

use crate::{cache::{LoggerCache, ShardManagerCache}, services::logger::LogType};

pub async fn message(ctx: Context, new_message: Message, options: &FrameworkOptions<(), Error>) {
    let data = ctx.data.read().await;

    { // * Logger scope
        if let Some(log) = data.get::<LoggerCache>() {
            let logger = log.read().await;
            logger.message(LogType::Info, &new_message);
        };
    }

    if let Some(shard_manager) = data.get::<ShardManagerCache>() {
        let framework_data = FrameworkContext {
            bot_id: ctx.cache.current_user_id(),
            user_data: &(),
            options,
            shard_manager
        };

        dispatch_event(framework_data, &ctx, &Event::Message { new_message }).await;
    }
}