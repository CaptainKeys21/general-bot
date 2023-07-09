use poise::serenity_prelude::{UserId, GuildId};
use serde::{Serialize, Deserialize};
use bson::{
    serde_helpers::chrono_datetime_as_bson_datetime,
    doc, to_document, Bson
};
use chrono::prelude::*;
use mongodb::error::Error;
use crate::{services::mongodb::Mongodb, utils::gb_serializer::Serializer};

#[derive(Serialize, Deserialize, Clone)]
pub struct MemberNote {
    member: UserId,
    guild: Option<GuildId>,
    by: UserId,
    index: u64,
    content: String,
    deleted: Option<Deleted>,
    edited: Vec<Edited>,
    #[serde(with="chrono_datetime_as_bson_datetime")]
    time: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Deleted {
    by: UserId,
    #[serde(with="chrono_datetime_as_bson_datetime")]
    time: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Edited {
    by: UserId,
    old_content: String,
    #[serde(with="chrono_datetime_as_bson_datetime")]
    time: DateTime<Utc>,
}

impl MemberNote {
    const DB_NAME: &str = "GeneralBot";
    const COLL_NAME: &str = "notes";

    pub async fn new_entry(database: &Mongodb, member: UserId, guild: Option<GuildId>, by: UserId, content: String) -> Result<MemberNote, Error> {
        let index = 
            database.count_collection_data(
                Self::DB_NAME, 
                Self::COLL_NAME, 
                Some(
                    doc! { 
                        "member": member.serialize(Serializer::new())? 
                    }
                )
            ).await?;

        let new_entry = MemberNote {
            index,
            member,
            guild,
            by,
            content,
            time: Utc::now(),
            deleted: None,
            edited: Vec::new(),
        };

        let entry_doc = new_entry.serialize(Serializer::new())?;

        database.insert_one(Self::DB_NAME, Self::COLL_NAME, to_document(&entry_doc)?).await?;

        Ok(new_entry)
    }

    pub async fn get_all_from_member(database: &Mongodb, member: UserId, full: bool) -> Result<Vec<MemberNote>, Error> {
        let mut filter = doc! { "member": member.serialize(Serializer::new())? };
        if !full {
            filter.insert("deleted", Bson::Null);
        }

        let result = database.get::<MemberNote>(Self::DB_NAME, Self::COLL_NAME, filter).await?;

        Ok(result)
    }

    pub async fn get_one_from_member(database: &Mongodb, member: UserId, guild: Option<GuildId>, index: u64) -> Result<Option<MemberNote>, Error> {
        let filter = doc! {
            "member": member.serialize(Serializer::new())?,
            "guild": guild.serialize(Serializer::new())?,
            "index": index.serialize(Serializer::new())?,
            "deleted": Bson::Null,
        };

        let data = database.get_one::<MemberNote>(Self::DB_NAME, Self::COLL_NAME, filter, None).await?;

        Ok(data)
    }

    pub async fn clear(database: &Mongodb, member: UserId, by: UserId) -> Result<u64, Error> {
        let del_entry = Deleted {
            by,
            time: Utc::now(),
        };

        let update = doc! {
            "$set": {
                "deleted": del_entry.serialize(Serializer::new())?
            }
        };

        let query = doc! {
            "member": member.serialize(Serializer::new())?,
            "deleted": Bson::Null
        };

        let res = database.update_many(Self::DB_NAME, Self::COLL_NAME, query, update).await?;
        Ok(res.modified_count)
    }

    pub async fn delete(&self, database: &Mongodb, by: UserId) -> Result<(), Error> {
        let del_entry = Deleted {
            by,
            time: Utc::now(),
        };

        let update = doc! {
            "$set": {
                "deleted": del_entry.serialize(Serializer::new())?
            }
        };

        let query = doc! {
            "index": self.index.serialize(Serializer::new())?,
            "member": self.member.serialize(Serializer::new())?,
            "guild": self.guild.serialize(Serializer::new())?
        };

        database.update_one::<MemberNote>(Self::DB_NAME, Self::COLL_NAME, query, update).await?;

        Ok(())
    }

    pub async fn edit(&self, database: &Mongodb, content: String, by: UserId) -> Result<(), Error> {
        let edited = Edited {
            by,
            old_content: self.content.clone(),
            time: Utc::now(),
        };

        let update = doc! {
            "$set": {
                "content": content
            },
            "$push": {
                "edited": edited.serialize(Serializer::new())?
            }
        };

        let query = doc! {
            "index": self.index.serialize(Serializer::new())?,
            "member": self.member.serialize(Serializer::new())?,
            "guild": self.guild.serialize(Serializer::new())?
        };

        database.update_one::<MemberNote>(Self::DB_NAME, Self::COLL_NAME, query, update).await?;

        Ok(())
    }

    pub fn index(&self) -> u64 {
        self.index + 1
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}