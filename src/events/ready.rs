use serenity::{
    model::prelude::Ready,
    prelude::Context,
};

pub async fn ready(ctx: Context, _ready: Ready) {
    println!("[Shard {}] Pronto", ctx.shard_id);       
}
