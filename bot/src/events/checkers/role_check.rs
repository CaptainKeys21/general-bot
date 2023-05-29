use serenity::{
    framework::standard::{
        macros::check, 
        Args, 
        CommandOptions, 
        Reason
    }, 
    prelude::Context, 
    model::prelude::Message
};

use crate::{
    cache::{
        LoggerCache,
        ConfigManagerCache
    }, 
    models::{ 
        configs::cmd_allowed_ids::CmdAllowedIds
    },
    services::logger::{
        LogType::*,
        CmdOrInt::*,
    }
};

#[check]
#[name = "Role"]
pub async fn role_check(ctx: &Context, msg: &Message, _args: &mut Args, cmd_opts: &CommandOptions) -> Result<(), Reason> {
    let data = ctx.data.read().await;

    //Getting logger
    let logger = data.get::<LoggerCache>();

    //Getting configs
    let cfg_manager = match data.get::<ConfigManagerCache>() {
        Some(m) => m.read().await,
        None => {
            match logger {
                Some(log) => {
                    log.read().await.command(Waring, cmd_opts.names[0], Command(&msg), Some("Config Manager not found"));
                },
                None => {
                    log::warn!("Config Manager not found");
                }
            };

            return Ok(());
        }
    };

    let author_roles = match &msg.member {
        Some(member) => member.roles.clone(),
        None => Vec::new(),
    };

    let author_id = msg.author.id.0;


    let cmd_configs = cfg_manager.get_many::<CmdAllowedIds>(Some(cmd_opts.names)).await;
    
    let mut is_allowed = false;

    if cmd_configs.is_empty() { is_allowed = true };

    for (_key, config) in cmd_configs.iter() {
        let ids = config.get_all_ids();
        if ids.contains(&author_id) { is_allowed = true; }
        
        for role_id in &author_roles {
            if ids.contains(&role_id.0) { is_allowed = true; }
        }
    }

    if !is_allowed {
        return Err(Reason::User("Sem cargos necessários".to_string()));
    }


    Ok(())
}