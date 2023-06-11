use poise::{
    FrameworkError, 
    BoxFuture
};

use serenity::Error;

use crate::{
    utils::embeds::{
        build_fail_embed, 
        build_warn_embed
    }, 
};

pub fn on_error(error: FrameworkError<'_, (), Error>) -> BoxFuture<'_, ()> {
    Box::pin(async move {
        match error {
            FrameworkError::Command { ctx, error } => {
                if let Err(e) = ctx.send(|m| { 
                    m.embeds.push(build_fail_embed(&ctx.author(), &error.to_string())); 
                    m.ephemeral(true);
                    m
                }).await {
                    log::error!("{}",e);
                };
            }
            FrameworkError::CommandPanic { ctx, payload: _ } => {
                // Not showing the payload to the user because it may contain sensitive info
                if let Err(e) = ctx.send(|m| { 
                    m.embeds.push(build_fail_embed(&ctx.author(), "Erro interno do servidor")); 
                    m.ephemeral(true);
                    m
                }).await {
                    log::error!("{}",e);
                };
            }
            FrameworkError::ArgumentParse { ctx, input, error} => {
                let usage = match ctx.command().help_text {
                    Some(help_text) => help_text(),
                    None => "Please check the help menu for usage information".into(),
                };
                let response = if let Some(input) = input {
                    format!(
                        "**Cannot parse `{}` as argument: {}**\n{}",
                        input, error, usage
                    )
                } else {
                    format!("**{}**\n{}", error, usage)
                };
                if let Err(e) = ctx.send(|m| { 
                    m.embeds.push(build_fail_embed(&ctx.author(), &response)); 
                    m.ephemeral(true);
                    m
                }).await {
                    log::error!("{}",e);
                };
            },
            FrameworkError::CooldownHit {
                remaining_cooldown,
                ctx,
            } => {
                if let Err(e) = ctx.send(|m| { 
                    m.embeds.push(build_fail_embed(&ctx.author(), &format!("Por favor, espere {} segundos para inserir o próximo comando", remaining_cooldown.as_secs()))); 
                    m.ephemeral(true);
                    m
                }).await {
                    log::error!("{}",e);
                };
            }
            FrameworkError::MissingBotPermissions {
                missing_permissions,
                ctx,
            } => {
                if let Err(e) = ctx.send(|m| { 
                    m.embeds.push(build_fail_embed(&ctx.author(), &format!("Comando não pode ser executado sem as seguintes permissões: {}", missing_permissions))); 
                    m.ephemeral(true);
                    m
                }).await {
                    log::error!("{}",e);
                };
            }
            FrameworkError::MissingUserPermissions {
                missing_permissions,
                ctx,
            } => {
                let response = if let Some(missing_permissions) = missing_permissions {
                    format!(
                        "Você está sem permissões para executar `{}{}`: {}",
                        ctx.prefix(),
                        ctx.command().name,
                        missing_permissions,
                    )
                } else {
                    format!(
                        "Você está sem permissões para executar`{}{}`",
                        ctx.prefix(),
                        ctx.command().name,
                    )
                };
                if let Err(e) = ctx.send(|m| { 
                    m.embeds.push(build_fail_embed(&ctx.author(), &response)); 
                    m.ephemeral(true);
                    m
                }).await {
                    log::error!("{}",e);
                };
            }
            FrameworkError::NotAnOwner { ctx } => {
                if let Err(e) = ctx.send(|m| { 
                    m.embeds.push(build_fail_embed(&ctx.author(), "Apenas o dono do bot pode usar esse comando")); 
                    m.ephemeral(true);
                    m
                }).await {
                    log::error!("{}",e);
                };
            }
            FrameworkError::GuildOnly { ctx } => {
                if let Err(e) = ctx.send(|m| { 
                    m.embeds.push(build_fail_embed(&ctx.author(), "Não pode usar esse comando em DMs")); 
                    m.ephemeral(true);
                    m
                }).await {
                    log::error!("{}",e);
                };
            }
            FrameworkError::DmOnly { ctx } => {
                if let Err(e) = ctx.send(|m| { 
                    m.embeds.push(build_fail_embed(&ctx.author(), "Comando só pode ser utilizado em DMs")); 
                    m.ephemeral(true);
                    m
                }).await {
                    log::error!("{}",e);
                };
            }
            FrameworkError::NsfwOnly { ctx } => {
                if let Err(e) = ctx.send(|m| { 
                    m.embeds.push(build_fail_embed(&ctx.author(), "Comando só pode ser usado em canais NSFW")); 
                    m.ephemeral(true);
                    m
                }).await {
                    log::error!("{}",e);
                };
            }
            FrameworkError::DynamicPrefix { error, msg, .. } => {
                log::error!(
                    "Dynamic prefix failed for message {:?}: {}",
                    msg.content,
                    error
                );
            }
            other => poise::builtins::on_error(other).await.unwrap(),
        }
    })
}
