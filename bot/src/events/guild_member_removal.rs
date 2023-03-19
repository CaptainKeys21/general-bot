use serenity::{
    prelude::Context, model::{
        prelude::{
            GuildId, 
            Member
        }, 
        user::User
    }
};

use crate::{cache::{LoggerCache, DatabaseCache}, models::member::BotMember};

pub async fn guild_member_removal(ctx: Context, guild_id: GuildId, user: User, member: Option<Member>) {
    let data = ctx.data.read().await;
    let log = data.get::<LoggerCache>().unwrap().read().await;
    let database = data.get::<DatabaseCache>().unwrap().read().await;

    if let Err(e) = BotMember::remove_one(&database, user.id).await {};
}