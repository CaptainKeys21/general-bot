use poise::{Context, command};
use serenity::{
    model::guild::Member, http::CacheHttp,
    Error,
};

use crate::{
    models::{
        context::ContextDataGetters,
        punishments::{
            GeneralBotPunishments,
            PunishManager,
            kick::MemberKick,
        }, 
    }, 
    utils::constants::COLOR_FAIL, 
    services::logger::LogType
};

fn help() -> String {
    String::from("\
    :information_source: Expulsa um usuário do servidor esse podendo voltar logo em seguida!
    :yellow_circle: Deve ser colocado a razão da punição!
    ")
}


#[command(prefix_command, slash_command, guild_only, help_text_fn="help")]
pub async fn kick(
    ctx: Context<'_, (), Error>, 
    #[description = "Membro"]member: Member,
    #[description = "Motivo"]reason: String
) -> Result<(), Error> {
    let data = ctx.serenity_context().data.read().await;
    let (database, logger) = data.get_essentials().await?;
    
    member.kick_with_reason(ctx.http(), &reason).await?;
    
    let new_kick = MemberKick::new(ctx.guild_id(), member.user.id, reason, ctx.author().id);

    match PunishManager::new_entry::<MemberKick>(&database, new_kick.clone()).await {
        Ok(_) => {
            ctx.send(|m| {
                m.embed(|e| 
                    e.title("Membro Banido")
                        .color(COLOR_FAIL)
                        .description(format!("<@{}>\n**Motivo:** {}", new_kick.member, new_kick.reason ))
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
