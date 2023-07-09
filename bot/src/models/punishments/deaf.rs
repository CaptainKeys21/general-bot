use poise::serenity_prelude::{UserId, GuildId};
use serde::{Serialize, Deserialize};
use bson::{
    serde_helpers::chrono_datetime_as_bson_datetime,
    doc, Bson
};
use chrono::prelude::*;
use std::error::Error;

use crate::{services::mongodb::Mongodb, utils::gb_serializer::Serializer};

use super::GeneralBotPunishments;

#[derive(Serialize, Deserialize, Clone)]
pub struct MemberDeafen {
    pub member: UserId,
    pub reason: String,
    guild_id: Option<GuildId>,
    by: UserId,
    #[serde(rename="type")]
    kind: String,
    #[serde(with="chrono_datetime_as_bson_datetime")]
    time: DateTime<Utc>,
    undeafen: Option<Undeafen>
}

#[derive(Serialize, Deserialize, Clone)]
struct Undeafen {
    by: UserId,
    #[serde(with="chrono_datetime_as_bson_datetime")]
    time: DateTime<Utc>,
    reason: Option<String>,
}

impl GeneralBotPunishments for MemberDeafen {
    type Data = MemberDeafen;
    const TYPE: &'static str = "deaf";
    fn new(guild_id: Option<GuildId>, member_id: UserId, reason: String, by_id: UserId) -> Self::Data {
        MemberDeafen {
            reason,
            member: member_id,
            by: by_id,
            guild_id: guild_id,
            kind: String::from(Self::TYPE),
            time: Utc::now(),
            undeafen: None,
        }
    }
}

impl MemberDeafen {
    pub async fn undeafen(&mut self, database: &Mongodb, by_id: UserId, reason: Option<String>) -> Result<(), Box<dyn Error>> {
        let undeafen = Undeafen {
            by: by_id,
            time: Utc::now(),
            reason
        };

        let query = doc! {
            "member": self.member.serialize(Serializer::new())?,
            "undeafen": Bson::Null,
        };

        let update = doc! {
            "$set": {
                "undeafen": undeafen.serialize(Serializer::new())?
            }
        };

        database.update_one::<MemberDeafen>("Logger", "punishment", query, update).await?;

        Ok(())
    }
}