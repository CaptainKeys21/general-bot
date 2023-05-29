use serenity::{prelude::Context, model::prelude::Message};

use crate::{cache::LoggerCache, services::logger::LogType};

pub async fn message(ctx: Context, new_message: Message) {
    let data = ctx.data.read().await;
    if let Some(log) = data.get::<LoggerCache>() {
        let logger = log.read().await;
        logger.message(LogType::Info, &new_message);
    };
    
}