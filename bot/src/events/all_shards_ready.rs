use serenity::{
    prelude::Context,
    async_trait,
};

use crate::{
    events::Handler,
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
    }
}