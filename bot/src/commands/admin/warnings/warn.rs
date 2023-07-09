use poise::{Context, command};
use serenity::{
    model::guild::Member,
    Error, http::CacheHttp,
};

use crate::{
    cache::{
        DatabaseCache, 
        LoggerCache
    }, 
    models::punishments::{
        PunishManager,
        warn::MemberWarn, 
        GeneralBotPunishments,
    }, 
    services::logger::LogType, utils::constants::COLOR_WARN
};

fn help() -> String {
    String::from("\
    :information_source: Dá um aviso sério no usuário!
    :yellow_circle: Deve ser colocado a razão da punição!
    :yellow_circle: Deve ser somente avisado no chat se não for algo sério, warnings tem punições graves conforme vão stackando no usuário!
    ")
}


#[command(prefix_command, slash_command, guild_only, help_text_fn="help")]
pub async fn warn(
    ctx: Context<'_, (), Error>, 
    #[description = "Membro"]member: Member,
    #[description = "Motivo"]reason: String,
    #[description = "Publico"]#[flag]public: bool
) -> Result<(), Error> {
    let data = ctx.serenity_context().data.read().await;
    let database = data.get::<DatabaseCache>().unwrap().read().await;
    let logger = data.get::<LoggerCache>().unwrap().read().await;

    let new_warn = MemberWarn::new(ctx.guild_id(), member.user.id, reason, ctx.author().id);

    match PunishManager::new_entry::<MemberWarn>(&database, new_warn.clone()).await {
        Ok(_) => {
            ctx.send(|m| {
                m.embed(|e| {
                    e.title("Aviso")
                    .color(COLOR_WARN)
                    .description(format!("<@{}>\n**Motivo:** {}", new_warn.member, new_warn.reason))
                }).ephemeral(!public)
            }).await?;

            if !public {
                member.user.direct_message(ctx.http(), 
                |m| 
                    m.embed(|e| {
                        e.title("Aviso")
                        .color(COLOR_WARN)
                        .description(format!("<@{}>\n**Motivo:** {}", new_warn.member, new_warn.reason))
                    })
                ).await?;
            }
        }
        Err(error) => {
            logger.default(LogType::Error, &format!("{}", error));
        }
    };

    Ok(())
}
