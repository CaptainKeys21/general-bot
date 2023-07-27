use poise::{Context, command};
use serenity::{
    model::guild::Member, http::CacheHttp,
    Error,
};

use crate::{
    models::{
        context::ContextDataGetters,
        punishments::{
            PunishManager,
            softban::MemberSoftBan, GeneralBotPunishments,
        }, 
    }, 
    utils::constants::COLOR_FAIL, 
    services::logger::LogType
};

fn help() -> String {
    String::from("\
    :information_source: Bane o usuário e já desbane logo em seguida, fazendo com que as ultimas mensagens desse usuário sejam deletadas!
    :yellow_circle: Deve ser colocado a razão da punição!
    ")
}


#[command(prefix_command, slash_command, guild_only, help_text_fn="help")]
pub async fn softban(
    ctx: Context<'_, (), Error>, 
    #[description = "Membro"]member: Member,
    #[description = "Motivo"]reason: String,
    #[description = "Deletar X dias de mensagem (padrão 7)"]days_message: Option<u8>
) -> Result<(), Error> {
    let data = ctx.serenity_context().data.read().await;
    let (database, logger) = data.get_essentials().await?;
    
    member.ban_with_reason(ctx.http(), days_message.unwrap_or(7), &reason).await?;
    member.unban(ctx.http()).await?;

    let mut new_ban = MemberSoftBan::new(ctx.guild_id(), member.user.id, reason, ctx.author().id);
    new_ban.set_deleted_days(days_message.unwrap_or(7));

    match PunishManager::new_entry::<MemberSoftBan>(&database, new_ban.clone()).await {
        Ok(_) => {
            ctx.send(|m| {
                m.embed(|e| 
                    e.title("Membro Banido (softban)")
                        .color(COLOR_FAIL)
                        .description(format!("<@{}>\n**Motivo:** {}", new_ban.member, new_ban.reason ))
                );
                m
            }).await?;
        }
        Err(error) => {
            logger.default(LogType::Error, &format!("{}", error));
        }
    };

    Ok(())
}
