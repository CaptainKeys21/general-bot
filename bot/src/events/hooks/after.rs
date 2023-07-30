use poise::{
    BoxFuture, 
    Context, 

};
use serenity::Error;

use crate::{
    services::logger::LogType::*,
    models::context::ContextDataGetters,
};

pub fn post_command(ctx: Context<'_, (), Error>) -> BoxFuture<'_, ()> {
    Box::pin(async move {
        let data = ctx.serenity_context().data.read().await;
        if let Ok ((_, logger)) = data.get_essentials().await {
            logger.command(Info, &ctx, Some("END"));
        };
    })
}