use poise::{Context, command};
use serenity::{
    model::guild::Member, 
    http::CacheHttp,
    Error,
};

use crate::{
    models::{punishments::{
        PunishManager,
        mute::MemberMute,
    }, context::ContextDataGetters}, 
    utils::constants::COLOR_OKAY, 
    services::logger::LogType
};

fn help() -> String {
    String::from("\
    :information_source: Expulsa um usuário do servidor esse podendo voltar logo em seguida!
    :yellow_circle: Deve ser colocado a razão da punição!
    ")
}


#[command(prefix_command, slash_command, guild_only, help_text_fn="help")]
pub async fn unmute(
    ctx: Context<'_, (), Error>, 
    #[description = "Membro"]mut member: Member,
    #[description = "Motivo"]#[lazy]reason: Option<String>,
) -> Result<(), Error> {
    let data = ctx.serenity_context().data.read().await;
    let (database, logger) = data.get_essentials().await?;

    member.enable_communication(ctx.http()).await?;

    match PunishManager::get_latest_by_member::<MemberMute>(&database, member.user.id).await {
        Ok(new_unmute) => {
            let member_id = match new_unmute {
                Some(mut u) => {
                    if let Err(e) = u.unmute(&database, ctx.author().id, reason).await {
                        logger.default(LogType::Error, &format!("{}", e))
                    };
                    u.member
                },
                None => member.user.id,
            };

            ctx.send(|m| {
                m.embed(|e| 
                    e.title("Membro Desmutado")
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

    Ok(())
}
