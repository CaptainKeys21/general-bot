use serenity::{
    prelude::Context, 
    model::prelude::Member
};

use crate::{
    cache::{
        LoggerCache, 
        DatabaseCache
    }, 
    models::member::BotMember,
    services::logger::LogType::Error
};

pub async fn guild_member_addition(ctx: Context, new_member: Member) {
    let data = ctx.data.read().await;
    let log = data.get::<LoggerCache>();

    let database = match data.get::<DatabaseCache>() {
        Some(d) => d.read().await,
        None => {
            if let Some(log) = log {
                log.read().await.default(Error, "Database not found");
            }
            return;
        }
    };

    if let Err(why) = BotMember::add_one(&database, new_member).await {
        log::error!("Database error: {}", why);
    };


}