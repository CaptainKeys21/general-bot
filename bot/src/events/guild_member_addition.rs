use serenity::{
    prelude::Context, 
    model::prelude::Member
};

use crate::{cache::{LoggerCache, DatabaseCache}, models::member::BotMember};

pub async fn guild_member_addition(ctx: Context, new_member: Member) {
    let data = ctx.data.read().await;
    let log = data.get::<LoggerCache>().unwrap().read().await;
    let database = data.get::<DatabaseCache>().unwrap().read().await;

    if let Err(e) = BotMember::add_one(&database, new_member).await {};
}