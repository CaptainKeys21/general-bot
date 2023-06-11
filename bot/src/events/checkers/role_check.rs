use poise::Context;
use serenity::Error;


use crate::{
    cache::{
        LoggerCache,
        ConfigManagerCache
    }, 
    models::{ 
        configs::cmd_allowed_ids::CmdAllowedIds
    },
    services::logger::LogType
};


pub async fn role_check(ctx: Context<'_, (), Error>) -> Result<bool, Error> {
    let data = ctx.serenity_context().data.read().await;

    //Getting logger
    let logger = data.get::<LoggerCache>();

    //Getting configs
    let cfg_manager = match data.get::<ConfigManagerCache>() {
        Some(m) => m.read().await,
        None => {
            match logger {
                Some(log) => {
                    log.read().await.default(LogType::Error, "Config Manager not found");
                },
                None => {
                    log::warn!("Config Manager not found");
                }
            };

            return Ok(true);
        }
    };

    let author_roles = match ctx.author_member().await {
        Some(member) => member.roles.clone(),
        None => Vec::new(),
    };

    let author_id = ctx.author().id.0;

    let cmd_configs =  cfg_manager.get_many::<CmdAllowedIds>(Some(&[&ctx.command().name])).await; 

    let mut is_allowed = false;

    if cmd_configs.is_empty() { is_allowed = true };

    for (_key, config) in cmd_configs.iter() {
        let ids = config.get_all_ids();
        if ids.contains(&author_id) { is_allowed = true; }
        
        for role_id in &author_roles {
            if ids.contains(&role_id.0) { is_allowed = true; }
        }
    }

    Ok(is_allowed)
}