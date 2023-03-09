use crate::utils::embeds::build_fail_embed;
use crate::models::member::BotMember;
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
    prelude::*,
};

#[command]
pub async fn insert_users(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    match msg.guild_id {
        Some(id) => {
            let emb = match BotMember::insert_all(&ctx, id.0).await {
                Ok(_) => build_fail_embed(&msg.author, "Dados inseridos com sucesso"),
                Err(_) => build_fail_embed(&msg.author, "Erro aos inserir os dados"),
            };
            
            msg.channel_id.send_message(&ctx.http, |m| m.set_embed(emb)).await?;
            Ok(())
        }
        None => {
            let emb = build_fail_embed(&msg.author, "Id do servidor não encontrado");
            msg.channel_id.send_message(&ctx.http, |m| m.set_embed(emb)).await?;

            Ok(())
        }
    }
}