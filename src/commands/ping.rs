use serenity::{
    framework::standard::{
        macros::command, 
        Args, 
        CommandResult
    },
    model::{
        prelude::*,
        interactions::application_command::ApplicationCommandInteraction,
        application::interaction::InteractionResponseType::ChannelMessageWithSource
    },
    prelude::*,
};


use std::time::Instant;

#[command]
pub async fn ping(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let old = Instant::now();
    let mut m = msg.channel_id.say(&ctx.http, "Pong!\n...").await?;
    let new = Instant::now();

    m.edit(ctx, |m| {
        m.content(format!("Pong!\n{} ms", (new - old).as_millis()))
    }).await?;

    Ok(())
}

pub async fn slash_ping(ctx: &Context, msg: &ApplicationCommandInteraction) -> CommandResult {
    let old = Instant::now();
    msg.create_interaction_response(&ctx.http, |resp| {
        resp.kind(ChannelMessageWithSource)
            .interaction_response_data(|data| data.content("Pong!\n.."))
    }).await?;

    let new = Instant::now();

    msg.edit_original_interaction_response(&ctx.http, |resp| {
        resp.content(format!("🏓 Pong!\n{} ms", (new - old).as_millis()))
    }).await?;

    Ok(())
}