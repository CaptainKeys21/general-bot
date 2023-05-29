use serenity::{
    prelude::Context, model::{
        prelude::{
            GuildId, 
            Member
        }, 
        user::User
    }
};

use crate::{
    cache::{
        LoggerCache, 
        DatabaseCache
    }, 
    models::member::BotMember,
    services::logger::LogType::Error
};

pub async fn guild_member_removal(ctx: Context, _guild_id: GuildId, user: User, _member: Option<Member>) {
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

    if let Err(why) = BotMember::remove_one(&database, user.id).await {
        log::error!("Database error: {}", why);
    };
}