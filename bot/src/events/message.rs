use serenity::{prelude::Context, model::prelude::Message};

use crate::{cache::LoggerCache, services::logger::LogType};

pub async fn message(ctx: Context, new_message: Message) {
    let data = ctx.data.read().await;
    let log = data.get::<LoggerCache>().unwrap().read().await;
    
    log.message(LogType::Info, &new_message)
}