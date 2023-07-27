use poise::{Context, command, serenity_prelude::UserId};
use serenity::{
    http::CacheHttp,
    Error,
};

use crate::{
    models::{
        context::ContextDataGetters,
        punishments::{
            PunishManager,
            ban::MemberBan
        }, 
    },
    utils::constants::COLOR_OKAY, 
    services::logger::LogType
};

fn help() -> String {
    String::from("\
    :information_source: Desilencia um usuário do servidor!
    :yellow_circle: Deve ser colocado a razão da punição e o tempo de punição!
    ")
}

#[command(prefix_command, slash_command, guild_only, help_text_fn="help")]
pub async fn unban(
    ctx: Context<'_, (), Error>, 
    #[description = "Membro"]user_id: UserId,
    #[description = "Motivo"]reason: String
) -> Result<(), Error> {
    let data = ctx.serenity_context().data.read().await;
    let (database, logger) = data.get_essentials().await?;

    if let Some(guild_id) = ctx.guild_id() {
        ctx.http().remove_ban(guild_id.0, user_id.0, Some(&reason)).await?;

        match PunishManager::get_latest_by_member::<MemberBan>(&database, user_id).await {
            Ok(new_unban) => {
                let member_id = match new_unban {
                    Some(mut u) => {
                        if let Err(e) = u.unban(&database, ctx.author().id, reason).await {
                            logger.default(LogType::Error, &format!("{}", e))
                        };
                        u.member
                    },
                    None => user_id,
                };

                ctx.send(|m| {
                    m.embed(|e| 
                        e.title("Membro Desbanido")
                            .color(COLOR_OKAY)
                            .description(format!("<@{}>", member_id))
                    );
                    m
                }).await?;
            },
            
            Err(error) => {
                logger.default(LogType::Error, &format!("{}", error));
            }
        };
    }

    Ok(())
}
