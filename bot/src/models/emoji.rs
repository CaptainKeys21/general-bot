use serde::{Deserialize, Serialize};
use serenity::prelude::Context;
use bson::Serializer;
use std::error::Error;

use crate::cache::DatabaseCache;

#[derive(Deserialize, Serialize)]
pub struct BotEmoji;

impl BotEmoji {
    pub async fn full_update(ctx: &Context, guild_id: u64) -> Result<(), Box<dyn Error>> {
        let data = ctx.data.read().await;
        let database = data.get::<DatabaseCache>().unwrap().read().await;

        database.clear_collection("GeneralBot", "emojis").await?;

        let emojis = match ctx.http.get_emojis(guild_id).await {
            Ok(emojis) => {
                let mut doc_list = Vec::new();
                for emojis in emojis.iter() {
                    if let Ok(serial_emojis) = emojis.serialize(Serializer::new()) {
                        if let Some(d) = serial_emojis.as_document() {
                            doc_list.push(d.clone());
                        }
                    }
                }

                doc_list
            }
            Err(_) => Vec::new()
        };

        database.insert_many("GeneralBot", "emojis", emojis).await?;

        Ok(())
    }
}