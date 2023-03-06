use serenity::{
    prelude::Context,
    async_trait,
};

use crate::{
    events::Handler,
    cache::CommandCache,
};

// Shard event handler
#[async_trait]
trait ShardsReadyHandler {
    async fn all_shards_ready(&self, ctx: &Context );
}

#[async_trait]
impl ShardsReadyHandler for Handler {
    async fn all_shards_ready(&self, ctx: &Context ) {
        let data = ctx.data.read().await;

        // register commands globally in release

        let mut cmd_mgr = data.get::<CommandCache>().unwrap().write().await;
        cmd_mgr.register_commands_global(ctx).await;
       
    }
}