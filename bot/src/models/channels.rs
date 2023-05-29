use serde::{Deserialize, Serialize};
use serenity::prelude::Context;
use bson::Serializer;
use std::error::Error;

use crate::cache::DatabaseCache;

#[derive(Deserialize, Serialize)]
pub struct BotChannels;

impl BotChannels {
    pub async fn full_update(ctx: &Context, guild_id: u64) -> Result<(), Box<dyn Error>> {
        let data = ctx.data.read().await;
        let database = match data.get::<DatabaseCache>() {
            Some(d) => d.read().await,
            None => return Err("database not found in cache".into()),
        };

        database.clear_collection("GeneralBot", "channels").await?;

        let channels = match ctx.http.get_channels(guild_id).await {
            Ok(channels) => {
                let mut doc_list = Vec::new();
                for channel in channels.iter() {
                    if let Ok(serial_channel) = channel.serialize(Serializer::new()) {
                        if let Some(d) = serial_channel.as_document() {
                            doc_list.push(d.clone());
                        }
                    }
                }

                doc_list
            }
            Err(_) => Vec::new()
        };

        database.insert_many("GeneralBot", "channels", channels).await?;

        Ok(())
    }
}