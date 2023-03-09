use std::collections::HashMap;
use futures::TryStreamExt;
use serde::{Deserialize, Serialize};
use bson::doc;

use crate::services::mongodb::Mongodb;

#[derive(Deserialize, Serialize)]
pub struct BotConfig {
    name: String,
    data: String,
}

impl BotConfig {
    pub async fn get_one(database: &Mongodb, config_name: &str) -> Option<String> {
        let collection = database.get_collection::<BotConfig>("GeneralBot", "config").await;

        let data = collection.find_one(doc! {"name": config_name}, None).await.unwrap_or(None);

        match data {
            Some(d) => Some(d.name),
            None => None,
        }
    }

    pub async fn get_many(database: &Mongodb, configs: &[&str]) -> HashMap<String, String> {
        let collection = database.get_collection::<BotConfig>("GeneralBot", "config").await;
        
        let mut filter = Vec::new();
        for conf in configs.iter() {
            filter.push(doc! {"name": conf});
        }

        let data = collection.find(doc! {"$or": filter}, None).await;

        let mut hash_res: HashMap<String, String> = HashMap::new();

        if let Ok(cursor) = data {
            let cur_vec: Vec<BotConfig> = cursor.try_collect().await.unwrap_or(Vec::new());

            for conf in cur_vec {
                hash_res.insert(conf.name, conf.data);
            }
        }
       
        hash_res
    }
}