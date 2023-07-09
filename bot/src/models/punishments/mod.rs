use bson::{to_document, doc, to_bson, Bson};
use poise::serenity_prelude::{GuildId, UserId};
use serde::{de::DeserializeOwned, Serialize};
use mongodb::{error::Error, options::FindOneOptions};
use crate::{services::mongodb::Mongodb, utils::gb_serializer::Serializer};

pub mod ban;
pub mod softban;
pub mod kick;
pub mod mute;
pub mod warn;
pub mod deaf;

pub trait GeneralBotPunishments {
    type Data: 'static + DeserializeOwned + Serialize + Sync + Send + Unpin + Clone;
    const TYPE: &'static str;

    fn new(guild_id: Option<GuildId>, member_id: UserId, reason: String, by_id: UserId) -> Self::Data;
}

pub struct PunishManager;

impl PunishManager {
    const DB_NAME: &str = "Logger";
    const COLL_NAME: &str = "punishment";

    pub async fn new_entry<P: GeneralBotPunishments>(database: &Mongodb, data: P::Data) -> Result<(), Error> {
        let document_entry = to_document(&data.serialize(Serializer::new())?)?;
        database.insert_one(Self::DB_NAME, Self::COLL_NAME, document_entry).await?;
        Ok(())
    }

    pub async fn get_latest_by_member<P: GeneralBotPunishments>(database: &Mongodb, member_id: UserId) -> Result<Option<P::Data>, Error> {
        let filter = doc! {
            "member": to_bson(&member_id)?,
            "type": String::from(P::TYPE)
        };

        let options = FindOneOptions::builder().sort(doc! {"time": -1}).build();
        let data = database.get_one::<P::Data>(Self::DB_NAME, Self::COLL_NAME, filter, Some(options)).await?;
        Ok(data)
    }
    
    pub async fn get_all_from_member<P: GeneralBotPunishments>(database: &Mongodb, member_id: UserId, non_deleted: Option<&str>) -> Result<Vec<P::Data>, Error> {
        let mut filter = doc! { 
            "type": String::from(P::TYPE), 
            "member": member_id.serialize(Serializer::new())? 
        };

        if let Some(key) = non_deleted {
            filter.insert(key, Bson::Null);
        };

        database.get::<P::Data>(Self::DB_NAME, Self::COLL_NAME, filter).await
    }
}