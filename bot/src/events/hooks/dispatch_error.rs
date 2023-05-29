use std::sync::Arc;
use tokio::sync::RwLock;

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
    services::logger::{LogType::*, Logger}
};

#[hook]
pub async fn dispatch_error(ctx: &Context, msg: &Message, error: DispatchError, _: &str) {
    let data = ctx.data.read().await;
    let logger = data.get::<LoggerCache>();

    async fn error_log(logger: Option<&Arc<RwLock<Logger>>>, why: &str) {
        match logger {
            Some(l) => {
                let log = l.read().await;
                log.default(Error, why);
            },
            None => {
                log::error!("{}", why);
            }
        };
    }

    match error {
        DispatchError::Ratelimited(_) => {
            let emb = build_fail_embed(&msg.author, "Limite de tempo entre um e outro exedido.");
            if let Err(why) = msg.channel_id.send_message(&ctx.http, |m| m.set_embed(emb)).await {
                error_log(logger, &format!("{}", why)).await;
            };
        },
        DispatchError::LackingRole => {
            let emb = build_warn_embed(&msg.author, "Você não possui os cargos necessários para executar esse comando");
            if let Err(why) = msg.channel_id.send_message(&ctx.http, |m| m.set_embed(emb)).await {
                error_log(logger, &format!("{}", why)).await;
            };
        },
        DispatchError::CheckFailed(_str, reason) => {
            let message = match reason {
                Unknown => String::from("Erro de checagem desconhecida"),
                User(msg) => msg,
                Log(msg) => {
                    error_log(logger, &msg).await;
                    String::from("Erro de checagem desconhecida")
                },
                UserAndLog { user, log } => {
                    error_log(logger, &log).await;
                    user
                },
                _ => String::from("Erro de checagem desconhecida")
            };

            let emb = build_warn_embed(&msg.author, &message);
            if let Err(why) = msg.channel_id.send_message(&ctx.http, |m| m.set_embed(emb)).await {
                error_log(logger, &format!("{}", why)).await;
            };
        }
        _ => {
            let emb = build_fail_embed(&msg.author, "Erro desconhecido.");
            if let Err(why) = msg.channel_id.send_message(&ctx.http, |m| m.set_embed(emb)).await {
                error_log(logger, &format!("{}", why)).await;
            };
        }
    };
}