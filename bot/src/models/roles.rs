use mongodb::options::FindOneOptions;
use serde::{Deserialize, Serialize};
use serenity::{
    prelude::Context,
    model::guild::Role,
};
use bson::{Serializer, doc};
use std::error::Error;

use crate::{cache::DatabaseCache, services::mongodb::Mongodb};

use super::context::ContextDataGetters;

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

    pub async fn get_top_role_from_list(database: &Mongodb, ids: Vec<String>) -> Result<Option<Role>, Box<dyn Error>> {
        let filter = doc! {
            "id": { 
                "$in": ids
            }
        };

        let options = FindOneOptions::builder().sort(doc! { "position": -1 }).build();

        let result = database.get_one::<Role>("GeneralBot", "roles", filter, Some(options)).await?;

        Ok(result)
    }
}