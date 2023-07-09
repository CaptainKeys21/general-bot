use std::collections::HashSet;

use futures::future::BoxFuture;

use poise::{
    PartialContext, 
    serenity_prelude::UserId
};

use serenity::{
    Error as SerenityError, 
    http::Http
};

use crate::{
    cache::ConfigManagerCache, 
    models::configs::general::GeneralConfig
};

pub fn dynamic_prefix(ctx: PartialContext<'_, (), SerenityError>) -> BoxFuture<'_, Result<Option<String>, SerenityError>> {
    Box::pin(async move { 
        let data = ctx.serenity_context.data.read().await;
        let prefix = match data.get::<ConfigManagerCache>() {
            Some(cfg_manager) => cfg_manager.read().await.get_one::<GeneralConfig>("prefix").await,
            None => return Err(SerenityError::Other("Config Manager not found"))
        };


        let res = match prefix {
            Ok(d) => Some(d.data),
            Err(_) => None
        };

        Ok(res)
    })
}

pub async fn get_owners(http: &Http) -> HashSet<UserId> {
    match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();

            owners.insert(info.owner.id);

            if let Some(team) = info.team {
                for member in &team.members {
                    owners.insert(member.user.id);
                }
            }

            owners
        }
        Err(why) => {
            println!("Erro ao acessar informações do bot: {}", why);
            HashSet::new()
        }
    }
}