use std::{
    error::Error,
    sync::Arc,
};

use tokio::sync::{Mutex, RwLock, RwLockReadGuard};

use chrono::FixedOffset;

use serenity::{
    client::bridge::gateway::ShardManager,
    prelude::{TypeMap, TypeMapKey},
    Error as SerenityError,
    async_trait
};

use crate::{
    services::{
        mongodb::Mongodb,
        logger::Logger,
    }, 
    models::configs::{
        config_manager::ConfigManager, 
        logger_blocklist::LoggerBlocklist, general::GeneralConfig
    },
};

/** Caching **/

pub struct ConfigManagerCache;
impl TypeMapKey for ConfigManagerCache {
    type Value = Arc<RwLock<ConfigManager>>;
}

pub struct ShardManagerCache;
impl TypeMapKey for ShardManagerCache {
    type Value = Arc<Mutex<ShardManager>>;
}

pub struct DatabaseCache;
impl TypeMapKey for DatabaseCache {
    type Value = Arc<RwLock<Mongodb>>;
}

pub struct LoggerCache;
impl TypeMapKey for LoggerCache {
    type Value = Arc<RwLock<Logger>>;
}

pub async fn fill(
    data: Arc<RwLock<TypeMap>>,
    config_manager: ConfigManager,
    shard_manager: Arc<tokio::sync::Mutex<ShardManager>>,
    database: Mongodb,
) -> Result<(), Box<dyn Error>> {
    let mut data = data.write().await;

    data.insert::<ConfigManagerCache>(Arc::new(RwLock::new(config_manager)));

    data.insert::<ShardManagerCache>(shard_manager);

    data.insert::<DatabaseCache>(Arc::new(RwLock::new(database.clone())));

    let mut logger = Logger::new(database.clone());
    {
        if let Some(cfg_mngr) = data.get::<ConfigManagerCache>() {
            let c_manager = cfg_mngr.read().await;

            if let Ok(block_list) = c_manager.get_one::<LoggerBlocklist>(LoggerBlocklist::NAME).await {
                logger.update_blocklist(block_list.get_all_ids());
            };
        }
    }

    data.insert::<LoggerCache>(Arc::new(RwLock::new(logger)));

    Ok(())
}