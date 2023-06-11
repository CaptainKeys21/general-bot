use poise::{Context, BoxFuture};
use serenity::Error;

pub mod role_check;

use role_check::role_check;

pub fn command_check(ctx: Context<'_, (), Error>) -> BoxFuture<'_, Result<bool, Error>> {
    Box::pin(async move {
        role_check(ctx).await
    })
}