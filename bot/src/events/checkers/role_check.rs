use serenity::{
    framework::standard::{
        Reason, 
        macros::check, 
        Args, 
        CommandOptions
    }, 
    prelude::Context, 
    model::prelude::Message
};

use crate::{
    cache::{
        LoggerCache, 
        DatabaseCache
    }, 
    models::{
        command_config::CommandConfig, 
        traits::GetFromDataBase
    },
    services::logger::{
        LogType::*,
        CmdOrInt::*,
    }, utils::embeds::build_warn_embed
};

#[check]
#[name = "Role"]
pub async fn role_check(ctx: &Context, msg: &Message, _args: &mut Args, cmd_opts: &CommandOptions) -> Result<(), Reason> {
    let data = ctx.data.read().await;

    //Getting logger
    let log = data.get::<LoggerCache>().unwrap().read().await;

    //Getting database
    let database = data.get::<DatabaseCache>().unwrap().read().await;

    let author_roles = match &msg.member {
        Some(member) => member.roles.clone(),
        None => Vec::new(),
    };

    let author_id = msg.author.id.0;


    let cmd_configs = CommandConfig::get_many(&database, cmd_opts.names).await;
    
    let mut is_allowed = false;

    if cmd_configs.is_empty() { is_allowed = true };

    for (_key, config) in cmd_configs.iter() {
        let ids = config.get_allowed_ids();
        if ids.contains(&author_id) { is_allowed = true; }
        
        for role_id in &author_roles {
            if ids.contains(&role_id.0) { is_allowed = true; }
        }
    }

    if !is_allowed {
        log.command(Error, cmd_opts.names[0], Command(&msg), None).await;
        let emb = build_warn_embed(&msg.author, "Você não possui os cargos necessários para executar esse comando");
        msg.channel_id.send_message(&ctx.http, |m| m.set_embed(emb)).await.unwrap();
        return Err(Reason::User("Lacked owner permission".to_string()));
    }


    Ok(())
}