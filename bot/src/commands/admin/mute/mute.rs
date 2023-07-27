use chrono::Utc;
use poise::{Context, command};
use serenity::{
    model::{guild::Member, Timestamp}, http::CacheHttp,
    Error,
};

use crate::{
    models::{punishments::{
        GeneralBotPunishments,
        PunishManager,
        mute::MemberMute,
    }, context::ContextDataGetters}, 
    utils::{constants::COLOR_WARN, functions::time_string_to_seconds}, 
    services::logger::LogType
};

fn help() -> String {
    String::from("\
    :information_source: Silencia um usuário do servidor!
    :yellow_circle: Deve ser colocado a razão da punição e o tempo de punição!
    ")
}


#[command(prefix_command, slash_command, guild_only, help_text_fn="help")]
pub async fn mute(
    ctx: Context<'_, (), Error>, 
    #[description = "Membro"]mut member: Member,
    #[description = "Motivo"]reason: String,
    #[description = "Tempo (s/m/h/D/M/Y)"]#[rest]time: String,
) -> Result<(), Error> {
    let data = ctx.serenity_context().data.read().await;
    let (database, logger) = data.get_essentials().await?;
    let timezone = data.get_timezone().await?;

    let mute_duration = match time_string_to_seconds(time) {
        Ok(t) => t,
        Err(_) => return Err(Error::Format(std::fmt::Error))
    };

    let mute_timestamp = match Timestamp::from_unix_timestamp(Utc::now().timestamp() + mute_duration) {
        Ok(t) => t,
        Err(_) => return Err(Error::Other("Invalid Timestamp"))
    };

    member.disable_communication_until_datetime(ctx.http(), mute_timestamp).await?;

    logger.default(LogType::Debug, &format!("{}", mute_duration));

    let mut new_mute = MemberMute::new(ctx.guild_id(), member.user.id, reason, ctx.author().id);
    new_mute.set_duration(*mute_timestamp);

    match PunishManager::new_entry::<MemberMute>(&database, new_mute.clone()).await {
        Ok(_) => {
            ctx.send(|m| {
                m.embed(|e| 
                    e.title("Membro Mutado")
                        .color(COLOR_WARN)
                        .description(format!("<@{}> mutado até *{}*\n**Motivo:** {}", new_mute.member, new_mute.get_formated_duration(&timezone), new_mute.reason))
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
