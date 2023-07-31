use poise::Context;
use serenity::Error;


use crate::{ 
    models::{configs::cmd_allowed_ids::CmdAllowedIds, context::ContextDataGetters},
    services::logger::LogType
};


pub async fn role_check(ctx: Context<'_, (), Error>) -> Result<bool, Error> {
    let data = ctx.serenity_context().data.read().await;

    let (_, logger) = data.get_essentials().await?;

    //Getting configs
    let cfg_manager = data.get_config_manager().await?;

    let author_roles = match ctx.author_member().await {
        Some(member) => member.roles.clone(),
        None => Vec::new(),
    };

    let author_id = ctx.author().id.0.to_string();

    let cmd_configs = match cfg_manager.get_one::<CmdAllowedIds>(&ctx.command().name).await {
        Ok(c) => c.unwrap_or_default(),
        Err(e) => {
            logger.default(LogType::Error, &format!("Role Check | {}", e));

            CmdAllowedIds::default()
        }
    }; 

    let mut is_allowed = false;

    let ids = cmd_configs.get_all_ids();

    if ids.is_empty() { is_allowed = true };

    if ids.contains(&author_id) { is_allowed = true; }
        
    for role_id in &author_roles {
        if ids.contains(&role_id.0.to_string()) { is_allowed = true; }
    }


    Ok(is_allowed)
}