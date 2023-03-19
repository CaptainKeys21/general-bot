use serde::Serialize;
use serenity::{
    model::prelude::Guild,
    prelude::Context,
};
use crate::cache::CommandCache;
use bson::Serializer;


pub async fn guild_create(ctx: Context, guild: Guild, is_new: bool) {
    // let data = ctx.data.read().await;

    // // register commands globally in release
    // let mut cmd_mgr = data.get::<CommandCache>().unwrap().write().await;
    // cmd_mgr.register_commands_guild(&ctx, &guild).await;
}