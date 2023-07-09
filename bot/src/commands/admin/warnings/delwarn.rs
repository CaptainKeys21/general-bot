use poise::{Context, command};
use serenity::{
    model::guild::Member,
    Error,
};

use crate::{
    cache::{
        DatabaseCache, 
        LoggerCache
    }, 
    models::punishments::{
        PunishManager,
        warn::MemberWarn,
    }, 
    utils::constants::{COLOR_OKAY, COLOR_WARN}, 
    services::logger::LogType
};

fn help() -> String {
    String::from("\
    :information_source: Deleta um aviso de um usuário
    ")
}


#[command(prefix_command, slash_command, guild_only, help_text_fn="help")]
pub async fn delwarn(
    ctx: Context<'_, (), Error>, 
    #[description = "Membro"]member: Member,
    #[description = "Motivo"]#[lazy]reason: Option<String>,
) -> Result<(), Error> {
    let data = ctx.serenity_context().data.read().await;
    let database = data.get::<DatabaseCache>().unwrap().read().await;
    let logger = data.get::<LoggerCache>().unwrap().read().await;

    match PunishManager::get_latest_by_member::<MemberWarn>(&database, member.user.id).await {
        Ok(warn) => {
            match warn {
                Some(mut w) => {
                    if let Err(e) = w.delete(&database, ctx.author().id, reason).await {
                        logger.default(LogType::Error, &format!("{}", e))
                    };
                    ctx.send(|m| {
                        m.embed(|e| 
                            e.title("Aviso deletado")
                                .color(COLOR_OKAY)
                                .description(format!("**{}**\n{}", w.reason, w.time.format_localized("%x %X", chrono::Locale::pt_BR)))
                        );
                        m
                    }).await?;
                },
                None => {
                    ctx.send(|m| {
                        m.embed(|e| {
                            e.title("Não há avisos para este membro").color(COLOR_WARN)
                        })
                    }).await?;
                },
            };

        },
        Err(error) => {
            logger.default(LogType::Error, &format!("{}", error));
        }
    };

    Ok(())
}
