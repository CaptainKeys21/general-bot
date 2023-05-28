use std::collections::HashMap;
use bson::Document;
use mongodb::error::Error;
use serde::{de::DeserializeOwned, Serialize};
use serenity::async_trait;

use crate::services::mongodb::Mongodb;

pub trait GeneralBotConfig {
    type Data: 'static + DeserializeOwned + Serialize;
    const TYPE: &'static str;
}

#[async_trait]
pub trait GetFromDataBase {
    type Output;

    async fn get_one(database: &Mongodb, config_name: &str, config_type: Option<&str>) -> Option<Self::Output>;
    async fn get_many(database: &Mongodb, filter: &[&str], config_type: Option<&str>) -> HashMap<String, Self::Output>;
}

#[async_trait]
pub trait InsertIntoDataBase {
    type Input;

    async fn insert_one(database: &Mongodb, data: Self::Input) -> Result<(), Error>;
    async fn insert_many(database: &Mongodb, data: &[Self::Input]) -> Result<(), Error>;
}

#[async_trait]
pub trait UpdateFromDataBase {
    type Input;

    async fn edit_one(database: &Mongodb, data: Self::Input, filter: Document) -> Result<(), Error>;
}