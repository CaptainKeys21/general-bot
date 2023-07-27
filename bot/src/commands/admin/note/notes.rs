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
pub async fn notes(
    ctx: Context<'_, (), Error>, 
    #[description = "Membro"]member: Member,
) -> Result<(), Error> {
    let data = ctx.serenity_context().data.read().await;
    let (database, logger) = data.get_essentials().await?;

    let notes = match MemberNote::get_all_from_member(&database, member.user.id, false).await {
        Ok(n) => n,
        Err(error) => {
            logger.default(LogType::Error, &format!("{}", error));
            return Err(Error::Other("Não foi possivel buscar os dados do membro"));
        }
    };

    ctx.send(|m| {
        m.embed(|e| {
            e.title(format!("Notas de {}", member.display_name()));

            if notes.len() > 0 {
                for note in notes {
                    e.field(note.index(), note.content(), false);
                }
            } else {
                e.description("**Não há avisos registrados para esse membro**");
            }
            e
        })
    }).await?;
    Ok(())
}