use std::collections::HashMap;
use futures::TryStreamExt;
use serde::{Deserialize, Serialize};
use bson::doc;
use serenity::async_trait;

use crate::services::mongodb::Mongodb;

use super::traits::GetFromDataBase;

#[derive(Deserialize, Serialize)]
pub struct GeneralConfig {
    name: String,
    data: String,
}

#[async_trait]
impl GetFromDataBase for GeneralConfig {
    type Output = String;

    async fn get_one(database: &Mongodb, config_name: &str, config_type: Option<&str>) -> Option<Self::Output> {
        let collection = database.get_collection::<GeneralConfig>("GeneralBot", "config").await;

        let mut filter = doc! {"name": config_name};

        if let Some(c_type) = config_type {
            filter.insert("config_type", c_type);
        }

        let data = collection.find_one(filter, None).await.unwrap_or(None);

        match data {
            Some(d) => Some(d.data),
            None => None,
        }
    }

    async fn get_many(database: &Mongodb, configs: &[&str], config_type: Option<&str>) -> HashMap<String, Self::Output> {
        let collection = database.get_collection::<GeneralConfig>("GeneralBot", "config").await;
        
        let mut names = Vec::new();
        for conf in configs.iter() {
            names.push(doc! {"name": conf})   
        }

        let query = match config_type {
            Some(t) => doc! {"config_type": t, "$or": names},
            None => doc! {"$or": names}
        };

        let data = collection.find(query, None).await;

        let mut hash_res: HashMap<String, String> = HashMap::new();

        if let Ok(cursor) = data {
            let cur_vec: Vec<GeneralConfig> = cursor.try_collect().await.unwrap_or(Vec::new());

            for conf in cur_vec {
                hash_res.insert(conf.name, conf.data);
            }
        }
       
        hash_res
    }
}