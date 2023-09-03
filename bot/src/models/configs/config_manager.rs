use std::{
    collections::HashMap, 
    error::Error
};

use bson::{Document, from_bson, doc, Serializer as BsonSerializer};
use futures::stream::TryStreamExt;
use serde::Serialize;

use crate::{
    services::mongodb::Mongodb, 
    models::traits::GeneralBotConfig
};

pub struct ConfigManager {
    configs: HashMap<String, Document>,
    database: Mongodb,
}

impl ConfigManager {
    pub async fn new(database: Mongodb) -> ConfigManager {
        let mut manager = ConfigManager { 
            configs: HashMap::new(), 
            database 
        };

        if let Err(e) = manager.full_update().await {
            eprintln!("[Command Manager] Full_update error: {}", e);
        };
        
        manager
    }

    pub async fn get_one<C: GeneralBotConfig>(&self, name: &str) -> Result<Option<C::Data>, Box<dyn Error>> {
        let config_key = &self.make_config_key(name, C::TYPE);

        let hash_data = self.configs.get(config_key);

        let data = match hash_data {
            Some(data) => Some(from_bson::<C::Data>(data.into())?),
            None => {
                let filter = doc! {
                    "name": name,
                    "config_type": C::TYPE
                };
                let db_data = self.database.get_one::<Document>("GeneralBot", "config", filter, None).await?;

                match db_data {
                    Some(d) => Some(from_bson::<C::Data>(d.into())?),
                    None => None,
                }
            }
        };

        Ok(data)
    }

    pub async fn get_many<C: GeneralBotConfig>(&self, names: Option<&[&str]>) -> HashMap<String, C::Data> {
        let mut fetched_data: HashMap<String, C::Data> = HashMap::new();
        match names {
            Some(c_names) => {
                for name in c_names {
                    let config_key = self.make_config_key(name, C::TYPE);

                    let data = match self.configs.get(&config_key) {
                        Some(d) => d,
                        None => continue,
                    };

                    let struct_doc = match from_bson::<C::Data>(data.into()) {
                        Ok(d) => d,
                        Err(_) => continue,
                    };
                    
                    fetched_data.insert(config_key, struct_doc);
                }
            },
            None => {
                for (key, doc) in &self.configs {
                    if self.check_type_in_config_key(&key, C::TYPE) {
                        let data_struct = match from_bson::<C::Data>(doc.into()) {
                            Ok(d) => d,
                            Err(_) => continue,
                        };

                        fetched_data.insert(self.get_config_name_from_key(&key), data_struct);
                    }
                }
            }
        };

        fetched_data
    }

    pub fn get_all_configs(&self) -> HashMap<String, Document> {
        self.configs.clone()
    }

    pub async fn update_one<C: GeneralBotConfig>(&mut self, name: &str, data: C::Data) -> Result<(), Box<dyn Error>> {
        let query = doc! {
            "name": name.to_string(),
            "config_type": C::TYPE.to_string(),
        };

        let update = match data.serialize(BsonSerializer::new())?.as_document() {
            Some(doc) => doc! { "$set": doc.clone() },
            None => return Err("Document conversion error".into())
        };

        match self.database.update_or_insert_one("GeneralBot", "config", query, update).await? {
            Some(doc) => {
                self.configs.insert(self.make_config_key(name, C::TYPE), doc);
            },
            None => {
                let filter = doc! { "name": name, "config_type": C::TYPE };
                if let Some(doc) = self.database.get_one::<Document>("GeneralBot", "config", filter, None).await? {
                    self.configs.insert(self.make_config_key(name, C::TYPE), doc);
                };
            }
        };


        Ok(())
    }

    async fn full_update(&mut self) -> Result<(), Box<dyn Error>> {
        let collection = self.database.get_collection::<Document>("GeneralBot", "config").await;
        let mut cursor = collection.find(None, None).await?;

        while let Some(config) = cursor.try_next().await? {
            let name = config.get_str("name").unwrap_or("unknown");
            let config_type = config.get_str("config_type").unwrap_or("unknown");
            let config_key = self.make_config_key(name, config_type);

            self.configs.insert(config_key, config);            
        }

        Ok(())
    }

    fn make_config_key(&self, name: &str, config_type: &str) -> String {
        String::from(name) + ":" + config_type
    }

    fn check_type_in_config_key(&self, config_key: &str, config_type: &str) -> bool {
        let ck_vec: Vec<&str> = config_key.split(":").collect();
        ck_vec[ck_vec.len() -1] == config_type
    }

    fn get_config_name_from_key(&self, config_key: &str) -> String {
        let ck_vec: Vec<&str> = config_key.split(":").collect();
        String::from(ck_vec[0])
    }
}