use std::collections::HashMap;
use serenity::async_trait;

use crate::services::mongodb::Mongodb;


#[async_trait]
pub trait GetFromDataBase {
    type Output;

    async fn get_one(database: &Mongodb, config_name: &str, config_type: Option<&str>) -> Option<Self::Output>;
    // async fn insert_one(database: &Mongodb, data: Self::Output) -> Result<(), Error>;
    // async fn edit_one(database: &Mongodb, data: Self::Output, filter: Document) -> Result<(), Error>;
    // async fn delete_one(database: &Mongodb, filter: Document) -> Result<Self::Output, Error>;

    async fn get_many(database: &Mongodb, filter: &[&str], config_type: Option<&str>) -> HashMap<String, Self::Output>;
    // async fn insert_many(database: &Mongodb, data: Vec<Self::Output>) -> Result<(), Error>;
    // async fn delete_many(database: &Mongodb, filter: Document) -> Result<Vec<Self::Output>, Error>;
}