use std::collections::HashMap;
use mongodb::error::Error;
use futures::TryStreamExt;
use serde::{Deserialize, Serialize};
use bson::{doc, Document};
use serenity::async_trait;

use crate::services::mongodb::Mongodb;

use super::traits::{GetFromDataBase, UpdateFromDataBase};

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

#[async_trait]
impl UpdateFromDataBase for GeneralConfig {
    type Input = String;

    async fn edit_one(database: &Mongodb, data: Self::Input, filter: Document) -> Result<(), Error> {
        let collection = database.get_collection::<GeneralConfig>("GeneralBot", "config").await;

        let document_set = doc! {
            "$set": {
                "data": data,
            }
        };

        if let Err(e) = collection.update_one(filter, document_set, None).await {
            return Err(e);
        };

        Ok(())
    }
}