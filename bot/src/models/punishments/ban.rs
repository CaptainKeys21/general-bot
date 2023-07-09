use poise::serenity_prelude::{UserId, GuildId};
use serde::{Serialize, Deserialize};
use bson::{
    serde_helpers::chrono_datetime_as_bson_datetime,
    doc, Bson
};
use chrono::{DateTime, Utc};
use std::error::Error;

use crate::{services::mongodb::Mongodb, utils::gb_serializer::Serializer};

use super::GeneralBotPunishments;

#[derive(Serialize, Deserialize, Clone)]
pub struct MemberBan {
    pub member: UserId,
    pub reason: String,
    guild_id: Option<GuildId>,
    by: UserId,
    #[serde(rename="type")]
    kind: String,
    #[serde(with="chrono_datetime_as_bson_datetime")]
    time: DateTime<Utc>,
    unban: Option<Unban>,
}

#[derive(Serialize, Deserialize, Clone)]
struct Unban {
    #[serde(with="chrono_datetime_as_bson_datetime")]
    time: DateTime<Utc>,
    reason: String,
    by: UserId,
}

impl GeneralBotPunishments for MemberBan {
    type Data = MemberBan;
    const TYPE: &'static str = "ban";

    fn new(guild_id: Option<GuildId>, member_id: UserId, reason: String, by_id: UserId) -> Self::Data {
        MemberBan {
            reason,
            member: member_id,
            by: by_id,
            guild_id: guild_id,
            kind: String::from(Self::TYPE),
            time: Utc::now(),
            unban: None,
        }
    }
}

impl MemberBan {
    pub async fn unban(&mut self, database: &Mongodb, by: UserId, reason: String) -> Result<(), Box<dyn Error>> {
        let unban = Unban {
            time: Utc::now(),
            reason,
            by,
        };

        let query = doc! {
            "member": self.member.serialize(Serializer::new())?,
            "unban": Bson::Null,
        };

        let update = doc! {
            "$set": {
                "unban": unban.serialize(Serializer::new())?
            }
        };

        database.update_one::<MemberBan>("Logger", "bans", query, update).await?;

        Ok(())
    }
}