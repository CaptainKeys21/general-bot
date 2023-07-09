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
    models::notes::MemberNote, 
    services::logger::LogType, utils::constants::COLOR_WARN
};

fn help() -> String {
    String::from("\
    :information_source: Adciona uma nota ao usuário!
    :yellow_circle: Comandos de nota devem sempre ser usados em chat privado para não ficar a publico!
    ")
}


#[command(prefix_command, slash_command, guild_only, help_text_fn="help")]
pub async fn editnote(
    ctx: Context<'_, (), Error>, 
    #[description = "Membro"]member: Member,
    #[description = "Numero da nota"]index: u64,
    #[description = "Texto"]#[rest]content: String,
) -> Result<(), Error> {
    let data = ctx.serenity_context().data.read().await;
    let database = data.get::<DatabaseCache>().unwrap().read().await;
    let logger = data.get::<LoggerCache>().unwrap().read().await;

    let note = match MemberNote::get_one_from_member(&database, member.user.id, ctx.guild_id(), index - 1).await {
        Ok(n) => {
            match n {
                Some(sm) => sm,
                None => {
                    ctx.send(|m| 
                        m.embed(|e| 
                            e.title("Nota não encontrada").color(COLOR_WARN)
                        )
                    ).await?;
                    return Ok(());
                }
            }
        },
        Err(error) => {
            logger.default(LogType::Error, &format!("{}", error));
            return Err(Error::Other("Não foi possivel buscar os dados do membro"));
        }
    };

    if let Err(e) = note.edit(&database, content, ctx.author().id).await {
        logger.default(LogType::Error, &format!("{}", e));
        return Err(Error::Other("Erro ao deletar nota"));
    };

    ctx.send(|m| 
        m.embed(
            |e| e.title(format!("Nota {}, Editada.", note.index()))
        )
    ).await?;

    Ok(())
}
