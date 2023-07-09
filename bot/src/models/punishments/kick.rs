use poise::serenity_prelude::{UserId, GuildId};
use serde::{Serialize, Deserialize};
use bson::{
    serde_helpers::chrono_datetime_as_bson_datetime,
    doc
};
use chrono::{DateTime, Utc};

use super::GeneralBotPunishments;

#[derive(Serialize, Deserialize, Clone)]
pub struct MemberKick {
    pub member: UserId,
    pub reason: String,
    guild_id: Option<GuildId>,
    by: UserId,
    #[serde(rename="type")]
    kind: String,
    #[serde(with="chrono_datetime_as_bson_datetime")]
    time: DateTime<Utc>,
}

impl GeneralBotPunishments for MemberKick {
    type Data = MemberKick;
    const TYPE: &'static str = "kick";

    fn new(guild_id: Option<GuildId>, member_id: UserId, reason: String, by_id: UserId) -> Self::Data {
        MemberKick {
            reason,
            member: member_id,
            by: by_id,
            guild_id: guild_id,
            kind: String::from(Self::TYPE),
            time: Utc::now(),
        }
    }
}