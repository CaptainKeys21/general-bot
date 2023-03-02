use crate::{
    utils::constants::{
        COLOR_OKAY,
        CARGO_APP_NAME,
        CARGO_APP_VERSION,
    },
};
use serenity::{
    framework::standard::{
        macros::command,
        Args,
        CommandResult,
    },
    model::{
        prelude::*,
        application::interaction::{
            InteractionResponseType::ChannelMessageWithSource,
            application_command::ApplicationCommandInteraction
        }
    },
    builder::{
        CreateEmbed,
        CreateEmbedFooter,
    },
    prelude::*,
};


#[command]
pub async fn info(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let bot_info = ctx.http.get_current_application_info().await?;
    let mut embed = CreateEmbed::default();
    embed.title(bot_info.name);
    embed.color(COLOR_OKAY);
    embed.description("Teste");
    embed.field("Info", "Mostra essa mensagem.", false);
    embed.field("Ping", "Testa se o bot está online e o quão rápido é a resposta do comando.", false);
    
    let mut embed_footer = CreateEmbedFooter::default();
    embed_footer.text(format!("{CARGO_APP_NAME} | {CARGO_APP_VERSION}"));
    embed.set_footer(embed_footer);

    msg.channel_id.send_message(&ctx.http, |msg| msg.set_embed(embed)).await?;

    Ok(())
}