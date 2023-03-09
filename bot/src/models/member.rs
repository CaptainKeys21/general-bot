use serde::{Serialize, Deserialize};
use serenity::prelude::Context;
use std::error::Error;
use bson::{doc, Serializer};

use crate::cache::DatabaseCache;

#[derive(Deserialize, Serialize)]
pub struct BotMember;

impl BotMember {
    pub async fn insert_all(ctx: &Context, guild_id: u64) -> Result<(), Box<dyn Error>> {
        let data = ctx.data.read().await;
        let database = data.get::<DatabaseCache>().unwrap().read().await;

        let members = match ctx.http.get_guild_members(guild_id, None, None).await {
            Ok(members) => {
                let mut doc_list = Vec::new();
                for member in members.iter() {
                    if let Ok(serial_member) = member.serialize(Serializer::new()) {
                        doc_list.push(doc! { "data": serial_member });
                    }
                }

                doc_list
            }
            Err(_) => Vec::new()
        };

        database.insert_many("GeneralBot", "members", members).await?;

        Ok(())
    }
}