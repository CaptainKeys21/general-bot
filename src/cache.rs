use std::{
    collections::HashMap,
    error::Error,
    sync::Arc,
};

use tokio::sync::{Mutex, RwLock};

use serenity::{
    client::bridge::gateway::ShardManager,
    prelude::{TypeMap, TypeMapKey},
    model::{
        channel::Message,
    },
};

use crate::services::commands::CommandManager;
use crate::services::mongodb::Mongodb;
use crate::services::logger::Logger;
use lru_cache::LruCache;

/** Caching **/

pub struct ConfigCache;
impl TypeMapKey for ConfigCache {
    type Value = Arc<RwLock<HashMap<&'static str, String>>>;
}

pub struct ShardManagerCache;
impl TypeMapKey for ShardManagerCache {
    type Value = Arc<Mutex<ShardManager>>;
}

pub struct MessageCacheEntry {
    pub our_msg: Message,
    pub original_msg: Message,
}
impl MessageCacheEntry {
    pub fn new(our_msg: Message, original_msg: Message) -> Self {
        MessageCacheEntry {
            our_msg,
            original_msg,
        }
    }
}

pub struct MessageCache;
impl TypeMapKey for MessageCache {
    type Value = Arc<Mutex<LruCache<u64, MessageCacheEntry>>>;
}

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
    prefix: &str,
    id: u64,
    shard_manager: Arc<tokio::sync::Mutex<ShardManager>>,
) -> Result<(), Box<dyn Error>> {
    let mut data = data.write().await;
    let mut map = HashMap::<&str, String>::new();

    map.insert("BOT_PREFIX", String::from(prefix));
    map.insert("BOT_ID", id.to_string());
    data.insert::<ConfigCache>(Arc::new(RwLock::new(map)));

    data.insert::<ShardManagerCache>(shard_manager);

    let commands = CommandManager::new();
    data.insert::<CommandCache>(Arc::new(RwLock::new(commands)));


    let database = Mongodb::new().await;
    data.insert::<DatabaseCache>(Arc::new(RwLock::new(database.clone())));

    let logger = Logger::new(database.clone());
    data.insert::<LoggerCache>(Arc::new(RwLock::new(logger)));

    Ok(())
}