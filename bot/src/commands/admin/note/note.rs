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
    services::logger::LogType, utils::constants::COLOR_INFO
};

fn help() -> String {
    String::from("\
    :information_source: Adciona uma nota ao usuário!
    :yellow_circle: Comandos de nota devem sempre ser usados em chat privado para não ficar a publico!
    ")
}


#[command(prefix_command, slash_command, guild_only, help_text_fn="help")]
pub async fn note(
    ctx: Context<'_, (), Error>, 
    #[description = "Membro"]member: Member,
    #[description = "Texto"]#[rest]content: String,
) -> Result<(), Error> {
    let data = ctx.serenity_context().data.read().await;
    let database = data.get::<DatabaseCache>().unwrap().read().await;
    let logger = data.get::<LoggerCache>().unwrap().read().await;

    match MemberNote::new_entry(&database, member.user.id, ctx.guild_id(), ctx.author().id, content).await {
        Ok(new_note) => {
            ctx.send(|m| {
                m.embed(|e| {
                    e.title(format!("Nota {} adicionada para {}.", new_note.index(), member.display_name()))
                    .color(COLOR_INFO)
                    .description(new_note.content())
                })
            }).await?;
        },
        Err(error) => {
            logger.default(LogType::Error, &format!("{}", error));
        }
    };

    Ok(())
}
