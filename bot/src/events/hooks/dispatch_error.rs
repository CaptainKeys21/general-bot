use serenity::{
    framework::standard::{
        macros::hook,
        DispatchError,
        Reason::*,
    },
    prelude::Context,
    model::channel::Message
};

use crate::{
    utils::embeds::{
        build_fail_embed, 
        build_warn_embed
    }, 
    cache::LoggerCache,
    services::logger::LogType::*
};

#[hook]
pub async fn dispatch_error(ctx: &Context, msg: &Message, error: DispatchError, _: &str) {
    let data = ctx.data.read().await;
    let logger = data.get::<LoggerCache>().unwrap().read().await;

    match error {
        DispatchError::Ratelimited(_) => {
            let emb = build_fail_embed(&msg.author, "Limite de tempo entre um e outro exedido.");
            msg.channel_id.send_message(&ctx.http, |m| m.set_embed(emb)).await.unwrap();
        },
        DispatchError::LackingRole => {
            let emb = build_warn_embed(&msg.author, "Você não possui os cargos necessários para executar esse comando");
            msg.channel_id.send_message(&ctx.http, |m| m.set_embed(emb)).await.unwrap();
        },
        DispatchError::CheckFailed(_str, reason) => {
            let message = match reason {
                Unknown => String::from("Erro de checagem desconhecida"),
                User(msg) => msg,
                Log(msg) => {
                    logger.default(Error, &msg).await;
                    String::from("Erro de checagem desconhecida")
                },
                UserAndLog { user, log } => {
                    logger.default(Error, &log).await;
                    user
                },
                _ => String::from("Erro de checagem desconhecida")
            };

            let emb = build_warn_embed(&msg.author, &message);
            msg.channel_id.send_message(&ctx.http, |m| m.set_embed(emb)).await.unwrap();
        }
        _ => {
            let emb = build_fail_embed(&msg.author, "Erro desconhecido.");
            msg.channel_id.send_message(&ctx.http, |m| m.set_embed(emb)).await.unwrap();
        }
    }
}