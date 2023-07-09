use poise::serenity_prelude::{UserId, GuildId};
use serde::{Serialize, Deserialize};
use serde_with::serde_as;
use bson::{
    serde_helpers::chrono_datetime_as_bson_datetime,
    doc, Bson
};
use chrono::prelude::*;
use std::error::Error;

use crate::{services::mongodb::Mongodb, utils::gb_serializer::Serializer};

use super::GeneralBotPunishments;

#[serde_as]
#[derive(Serialize, Deserialize, Clone)]
pub struct MemberWarn {
    pub member: UserId,
    pub reason: String,
    guild_id: Option<GuildId>,
    by: UserId,
    #[serde(rename="type")]
    kind: String,
    #[serde(with="chrono_datetime_as_bson_datetime")]
    pub time: DateTime<Utc>,
    deleted: Option<Deleted>
}

#[derive(Serialize, Deserialize, Clone)]
struct Deleted {
    by: UserId,
    #[serde(with="chrono_datetime_as_bson_datetime")]
    time: DateTime<Utc>,
    reason: Option<String>,
}

impl GeneralBotPunishments for MemberWarn {
    type Data = MemberWarn;
    const TYPE: &'static str = "warning";
    fn new(guild_id: Option<GuildId>, member_id: UserId, reason: String, by_id: UserId) -> Self::Data {
        MemberWarn {
            reason,
            member: member_id,
            by: by_id,
            guild_id: guild_id,
            kind: String::from(Self::TYPE),
            time: Utc::now(),
            deleted: None
        }
    }
}

impl MemberWarn {
    pub async fn delete(&mut self, database: &Mongodb, by_id: UserId, reason: Option<String>) -> Result<(), Box<dyn Error>> {
        let deleted = Deleted {
            by: by_id,
            time: Utc::now(),
            reason
        };

        let query = doc! {
            "member": self.member.serialize(Serializer::new())?,
            "deleted": Bson::Null,
        };

        let update = doc! {
            "$set": {
                "deleted": deleted.serialize(Serializer::new())?
            }
        };

        database.update_one::<MemberWarn>("Logger", "punishment", query, update).await?;

        Ok(())
    }
}