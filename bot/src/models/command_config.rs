use std::collections::HashMap;

use futures::TryStreamExt;
use serde::{Deserialize, Serialize};
use serenity::async_trait;
use bson::doc;

use crate::services::mongodb::Mongodb;

use super::traits::GetFromDataBase;

#[derive(Deserialize, Serialize, Clone)]
pub struct CommandConfig {
    config_type: String,
    name: String,
    group: String,
    allowed: Allowed,
}

#[derive(Deserialize, Serialize, Clone)]
struct Allowed {
    roles: Vec<u64>,
    users: Vec<u64>,
}

impl CommandConfig {
    pub fn get_allowed_ids(&self) -> Vec<u64> {
        [&self.allowed.roles[..], &self.allowed.users[..]].concat()
    }
}

#[async_trait]
impl GetFromDataBase for CommandConfig {
    type Output = CommandConfig;

    async fn get_one(database: &Mongodb, config_name: &str, config_type:Option<&str>) -> Option<Self::Output> {
        let collection = database.get_collection::<Self::Output>("GeneralBot", "config").await;

        let mut filter = doc! {"name": config_name};

        if let Some(c_type) = config_type {
            filter.insert("config_type", c_type);
        }

        let data = collection.find_one(filter, None).await.unwrap_or(None);

        data
    }

    async fn get_many(database: &Mongodb, names: &[&str]) -> HashMap<String, Self::Output>{
        let collection = database.get_collection::<Self::Output>("GeneralBot", "config").await;
        
        let mut filter = Vec::new();
        for name in names.iter() {
            filter.push(doc! {"name": name});
        }

        let data = collection.find(doc! {"$or": filter}, None).await;

        let mut hash_res: HashMap<String, Self::Output> = HashMap::new();

        if let Ok(cursor) = data {
            let cur_vec: Vec<Self::Output> = cursor.try_collect().await.unwrap_or(Vec::new());

            for conf in cur_vec {
                hash_res.insert(conf.name.clone(), conf.clone());
            }
        }
       
        hash_res
    }
}