use poise::{Context, command};
use serenity::{
    model::guild::Member,
    Error,
};

use crate::{
    models::{notes::MemberNote, context::ContextDataGetters}, 
    services::logger::LogType,
};

fn help() -> String {
    String::from("\
    :information_source: Lista as notas de um usuário!
    :yellow_circle: Comandos de nota devem sempre ser usados em chat privado para não ficar a publico!
    ")
}


#[command(prefix_command, slash_command, guild_only, help_text_fn="help")]
pub async fn clearnotes(
    ctx: Context<'_, (), Error>, 
    #[description = "Membro"]member: Member,
) -> Result<(), Error> {
    let data = ctx.serenity_context().data.read().await;
    let (database, logger) = data.get_essentials().await?;

    let num_notes = match MemberNote::clear(&database, member.user.id, ctx.author().id).await {
        Ok(n) => n,
        Err(error) => {
            logger.default(LogType::Error, &format!("{}", error));
            return Err(Error::Other("Não foi possivel buscar os dados do membro"));
        }
    };

    ctx.send(|m| {
        m.embed(|e| {
            e.title(format!("{} notas de {} foram apagadas", num_notes, member.display_name()));
            e
        })
    }).await?;
    Ok(())
}