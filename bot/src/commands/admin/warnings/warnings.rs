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
    services::logger::LogType,
};

fn help() -> String {
    String::from("\
    :information_source: Verifica os avisos que um usuário levou!
    ")
}


#[command(prefix_command, slash_command, guild_only, help_text_fn="help")]
pub async fn warnings(
    ctx: Context<'_, (), Error>, 
    #[description = "Membro"]member: Member,
    #[description = "Incluir deletados"]#[flag]deep: bool
) -> Result<(), Error> {
    let data = ctx.serenity_context().data.read().await;
    let database = data.get::<DatabaseCache>().unwrap().read().await;
    let logger = data.get::<LoggerCache>().unwrap().read().await;

    let deep_filter = if !deep { Some("deleted") } else { None };

    let warnings = match PunishManager::get_all_from_member::<MemberWarn>(&database, member.user.id, deep_filter).await {
        Ok(v) => v,
        Err(e) => {
            logger.default(LogType::Error, &format!("{}", e));
            return Err(Error::Other("Não foi possivel buscar os dados do membro"));
        }
    };

    ctx.send(|m| {
        m.embed(|e| {
            e.title(format!("Avisos de {}", member.display_name()));

            if warnings.len() > 0 {
                for warn in warnings {
                    e.field(warn.reason, warn.time.format_localized("%x %X", chrono::Locale::pt_BR), false);
                }
            } else {
                e.description("**Não há avisos registrados para esse membro**");
            }
            e
        })
    }).await?;

    Ok(())
}
