use serenity::{
    model::prelude::Guild,
    prelude::Context,
};
use crate::{
    // cache::CommandCache, 
    models::{
        member::BotMember, 
        roles::BotRoles, 
        channels::BotChannels, 
        emoji::BotEmoji
    }
};


pub async fn guild_create(ctx: Context, guild: Guild, _is_new: bool) {
    // let data = ctx.data.read().await;

    // // register commands globally in release
    // let mut cmd_mgr = data.get::<CommandCache>().unwrap().write().await;
    // cmd_mgr.register_commands_guild(&ctx, &guild).await;

    if cfg!(debug_assertions) {
        if let Err(_) = BotMember::full_update(&ctx, guild.id.0).await {};
        if let Err(_) = BotRoles::full_update(&ctx, guild.id.0).await {};
        if let Err(_) = BotChannels::full_update(&ctx, guild.id.0).await {};
        if let Err(_) = BotEmoji::full_update(&ctx, guild.id.0).await {};
    }
}