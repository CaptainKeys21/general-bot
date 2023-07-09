use poise::serenity_prelude::{GuildId, UserId};
use serde::{Serialize, Deserialize};
use bson::{
    serde_helpers::chrono_datetime_as_bson_datetime,
    doc
};
use chrono::{DateTime, Utc};

use super::GeneralBotPunishments;

#[derive(Serialize, Deserialize, Clone)]
pub struct MemberSoftBan {
    pub member: UserId,
    pub reason: String,
    guild_id: Option<GuildId>,
    by: UserId,
    #[serde(rename="type")]
    kind: String,
    #[serde(with="chrono_datetime_as_bson_datetime")]
    time: DateTime<Utc>,
    deleted_days: u8,
}

impl GeneralBotPunishments for MemberSoftBan {
    type Data = MemberSoftBan;
    const TYPE: &'static str = "softban";

    fn new(guild_id: Option<GuildId>, member_id: UserId, reason: String, by_id: UserId) -> Self::Data {
        MemberSoftBan {
            reason,
            member: member_id,
            by: by_id,
            guild_id: guild_id,
            kind: String::from(Self::TYPE),
            time: Utc::now(),
            deleted_days: 7,
        }
    }
}

impl MemberSoftBan {
    pub fn set_deleted_days(&mut self, days: u8) {
        self.deleted_days = days;
    }
}