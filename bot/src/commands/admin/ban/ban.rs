use poise::{Context, command};
use serenity::{
    model::guild::Member, http::CacheHttp,
    Error,
};

use crate::{
    cache::{
        DatabaseCache, 
        LoggerCache
    }, 
    models::punishments::{
        GeneralBotPunishments,
        PunishManager,
        ban::MemberBan
    }, 
    utils::constants::COLOR_FAIL, 
    services::logger::LogType
};

fn help() -> String {
    String::from("\
    :information_source: Bane um usuário do servidor!
    :yellow_circle: Deve ser colocado a razão da punição!
    ")
}


#[command(prefix_command, slash_command, guild_only, help_text_fn="help")]
pub async fn ban(
    ctx: Context<'_, (), Error>, 
    #[description = "Membro"]member: Member,
    #[description = "Motivo"]reason: String
) -> Result<(), Error> {
    let data = ctx.serenity_context().data.read().await;
    let database = data.get::<DatabaseCache>().unwrap().read().await;
    let logger = data.get::<LoggerCache>().unwrap().read().await;

    member.ban_with_reason(ctx.http(), 7, &reason).await?;
        
    let new_ban = MemberBan::new(ctx.guild_id(), member.user.id, reason, ctx.author().id);
    match PunishManager::new_entry::<MemberBan>(&database, new_ban.clone()).await {
        Ok(_) => {
            ctx.send(|m| {
                m.embed(|e| 
                    e.title("Membro Banido")
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
