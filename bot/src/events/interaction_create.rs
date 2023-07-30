use poise::{FrameworkContext, FrameworkOptions, Event, dispatch_event};
use serenity::{
    prelude::Context, 
    model::prelude::interaction::Interaction,
    Error
};

use crate::cache::ShardManagerCache;



pub async fn interaction_create(ctx: Context, interaction: Interaction, options: &FrameworkOptions<(), Error>) {
    let data = ctx.data.read().await;

    if let Some(shard_manager) = data.get::<ShardManagerCache>() {
        let framework_data = FrameworkContext {
            bot_id: ctx.cache.current_user_id(),
            user_data: &(),
            options,
            shard_manager
        };

        dispatch_event(framework_data, &ctx, &Event::InteractionCreate { interaction }).await;
    }
}