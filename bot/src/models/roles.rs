use serde::{Deserialize, Serialize};
use serenity::prelude::Context;
use bson::Serializer;
use std::error::Error;

use crate::cache::DatabaseCache;

#[derive(Deserialize, Serialize)]
pub struct BotRoles;

impl BotRoles {
    pub async fn full_update(ctx: &Context, guild_id: u64) -> Result<(), Box<dyn Error>> {
        let data = ctx.data.read().await;
        let database = match data.get::<DatabaseCache>() {
            Some(d) => d.read().await,
            None => return Err("database not found in cache".into()),
        };

        database.clear_collection("GeneralBot", "roles").await?;

        let roles = match ctx.http.get_guild_roles(guild_id).await {
            Ok(roles) => {
                let mut doc_list = Vec::new();
                for role in roles.iter() {
                    if let Ok(serial_role) = role.serialize(Serializer::new()) {
                        if let Some(d) = serial_role.as_document() {
                            doc_list.push(d.clone());
                        }
                    }
                }

                doc_list
            }
            Err(_) => Vec::new()
        };

        database.insert_many("GeneralBot", "roles", roles).await?;

        Ok(())
    }
}