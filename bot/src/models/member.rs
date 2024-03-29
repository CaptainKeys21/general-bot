use serde::{Serialize, Deserialize};
use serenity::{prelude::Context, model::prelude::{Member, UserId}};
use std::error::Error;
use bson::{doc, Serializer};

use crate::{cache::DatabaseCache, services::mongodb::Mongodb};

#[derive(Deserialize, Serialize)]
pub struct BotMember;

impl BotMember {
    pub async fn full_update(ctx: &Context, guild_id: u64) -> Result<(), Box<dyn Error>> {
        let data = ctx.data.read().await;
        let database = match data.get::<DatabaseCache>() {
            Some(d) => d.read().await,
            None => return Err("database not found in cache".into()),
        };

        database.clear_collection("GeneralBot", "members").await?;

        let members = match ctx.http.get_guild_members(guild_id, None, None).await {
            Ok(members) => {
                let mut doc_list = Vec::new();
                for member in members.iter() {
                    if let Ok(serial_member) = member.serialize(Serializer::new()) {
                        if let Some(d) = serial_member.as_document() {
                            doc_list.push(d.clone());
                        }
                    }
                }

                doc_list
            }
            Err(_) => Vec::new()
        };

        database.insert_many("GeneralBot", "members", members).await?;

        Ok(())
    }

    pub async fn add_one(database: &Mongodb, member: Member) -> Result<(), Box<dyn Error>> {
        let serial_member = match member.serialize(Serializer::new()) {
            Ok(d) => doc! { "data": d },
            Err(e) => return Err(Box::new(e))
        };

        if let Err(e) = database.insert_one("GeneralBot", "members", serial_member).await {
            return Err(Box::new(e));
        };

        Ok(())
    }

    pub async fn remove_one(database: &Mongodb, member_id: UserId) -> Result<(), Box<dyn Error>> {
        let query = doc! {
            "data": {
                "user": {
                    "id": member_id.to_string(),
                }
            }
        };

        if let Err(e) = database.insert_one("GeneralBot", "members", query).await {
            return Err(Box::new(e));
        };

        Ok(())
    }
}