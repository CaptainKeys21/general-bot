use serenity::{
    framework::standard::{
        macros::hook,
        DispatchError,
    },
    prelude::Context,
    model::channel::Message
};

use crate::utils::embeds::build_fail_embed;

#[hook]
pub async fn dispatch_error(ctx: &Context, msg: &Message, error: DispatchError, _: &str) {
    match error {
        DispatchError::Ratelimited(_) => {
            let emb = build_fail_embed(&msg.author, "Limite de tempo entre um e outro exedido.");
            msg.channel_id.send_message(&ctx.http, |m| m.set_embed(emb)).await.unwrap();
        }
        _ => {
            let emb = build_fail_embed(&msg.author, "Erro desconhecido.");
            msg.channel_id.send_message(&ctx.http, |m| m.set_embed(emb)).await.unwrap();
        }
    }
}