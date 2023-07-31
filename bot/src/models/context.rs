use chrono::FixedOffset;
use tokio::sync::RwLockReadGuard;
use serenity::{
    prelude::TypeMap,
    async_trait,
    Error, 
};

use crate::{
    services::{
        logger::Logger,
        mongodb::Mongodb,
    },
    cache::{
        DatabaseCache,
        LoggerCache,
        ConfigManagerCache,
    },
    models::configs::{
        general::GeneralConfig,
        config_manager::ConfigManager,
    },
};

#[async_trait]
pub trait ContextDataGetters {
    async fn get_essentials(&self) -> Result<(RwLockReadGuard<'_, Mongodb>, RwLockReadGuard<'_, Logger>), Error>;
    async fn get_timezone(&self) -> Result<FixedOffset, Error>;
    async fn get_config_manager(&self) -> Result<RwLockReadGuard<'_, ConfigManager>, Error>;
}

#[async_trait]
impl ContextDataGetters for TypeMap {
    async fn get_essentials(&self) -> Result<(RwLockReadGuard<'_, Mongodb>, RwLockReadGuard<'_, Logger>), Error> {
        let db = match self.get::<DatabaseCache>() {
            Some(res) => res.read().await,
            None => return Err(Error::Other("Database not found")),
        };

        let log = match self.get::<LoggerCache>() {
            Some(res) => res.read().await,
            None => return Err(Error::Other("Logger not found")),
        };

        Ok((db, log))
    }

    async fn get_config_manager(&self) -> Result<RwLockReadGuard<'_, ConfigManager>, Error> {
        match self.get::<ConfigManagerCache>() {
            Some(c) => Ok(c.read().await),
            None => Err(Error::Other("Config manager not found")),
        }
    }

    async fn get_timezone(&self) -> Result<FixedOffset, Error> {
        let cfg_mngr = match self.get::<ConfigManagerCache>() {
            Some(res) => res.read().await,
            None => return Err(Error::Other("Config Managaer not found"))
        };

        let (hem, hours) = match cfg_mngr.get_one::<GeneralConfig>("timezone").await {
            Ok(res) => {
                let (mut h1, mut h2) = (String::from("W"), 0);

                if let Some(cfg) = res {
                    let mut split: Vec<&str> = cfg.data.split(":").collect();
    
                    if split.len() == 2 {
                        h1 = String::from(split.remove(0));
                        h2 = split.remove(0).parse::<i32>().unwrap_or(0);
                    }
                }

                (h1, h2)
            }
            Err(e) => {
                (String::from("W"), 0)
            }
        };

        match hem.as_str() {
            "W" => match FixedOffset::west_opt(hours) {
                Some(fo) => Ok(fo),
                None => Err(Error::Other("Timezone parsing error"))
            },
            "E" => match FixedOffset::east_opt(hours) {
                Some(fo) => Ok(fo),
                None => Err(Error::Other("Timezone parsing error"))
            },
            &_ => match FixedOffset::west_opt(hours) {
                Some(fo) => Ok(fo),
                None => Err(Error::Other("Timezone parsing error"))
            },
        }
    }
}