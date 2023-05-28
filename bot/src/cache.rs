use std::{
    error::Error,
    sync::Arc,
};

use tokio::sync::{Mutex, RwLock};

use serenity::{
    client::bridge::gateway::ShardManager,
    prelude::{TypeMap, TypeMapKey},
};

use crate::{
    services::{
        commands::CommandManager,
        mongodb::Mongodb,
        logger::Logger,
    }, 
    models::configs::config_manager::ConfigManager,
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

// pub struct MessageCacheEntry {
//     pub our_msg: Message,
//     pub original_msg: Message,
// }
// impl MessageCacheEntry {
//     pub fn new(our_msg: Message, original_msg: Message) -> Self {
//         MessageCacheEntry {
//             our_msg,
//             original_msg,
//         }
//     }
// }

// pub struct MessageCache;
// impl TypeMapKey for MessageCache {
//     type Value = Arc<Mutex<LruCache<u64, MessageCacheEntry>>>;
// }

pub struct CommandCache;
impl TypeMapKey for CommandCache {
    type Value = Arc<RwLock<CommandManager>>;
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

    //let commands = CommandManager::new();
    //data.insert::<CommandCache>(Arc::new(RwLock::new(commands)));

    data.insert::<DatabaseCache>(Arc::new(RwLock::new(database.clone())));

    let logger = Logger::new(database.clone());
    data.insert::<LoggerCache>(Arc::new(RwLock::new(logger)));

    Ok(())
}