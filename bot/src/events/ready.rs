use serenity::{
    model::prelude::Ready,
    prelude::Context,
    Error
};

use poise::{builtins::register_globally, FrameworkOptions};

use crate::{
    models::context::ContextDataGetters,
    services::logger::LogType
};

pub async fn ready(ctx: Context, _ready: Ready, options: &FrameworkOptions<(), Error>) {
    let data = ctx.data.read().await;
    let essentials = data.get_essentials().await;

    if let Err(e) = register_globally(ctx.http, &options.commands).await{
        match essentials {
            Ok((_, log)) => log.default(LogType::Error, &format!("Event Ready | {}", e)),
            Err(log_err) => {
                log::error!("Context Data | {}", log_err);
                log::error!("Event Ready | {}", e);
            }
        };
    };

    println!("[Shard {}] Pronto", ctx.shard_id);       
}
