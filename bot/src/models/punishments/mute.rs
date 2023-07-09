use poise::serenity_prelude::{UserId, GuildId};
use serde::{Serialize, Deserialize};
use serde_with::{serde_as};
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
pub struct MemberMute {
    pub member: UserId,
    pub reason: String,
    guild_id: Option<GuildId>,
    by: UserId,
    #[serde(rename="type")]
    kind: String,
    #[serde(with="chrono_datetime_as_bson_datetime")]
    time: DateTime<Utc>,
    #[serde(default)]
    duration: Option<DateTime<Utc>>,
    unmute: Option<Unmute>
}

#[derive(Serialize, Deserialize, Clone)]
struct Unmute {
    by: UserId,
    #[serde(with="chrono_datetime_as_bson_datetime")]
    time: DateTime<Utc>,
    reason: Option<String>,
}

impl GeneralBotPunishments for MemberMute {
    type Data = MemberMute;
    const TYPE: &'static str = "mute";
    fn new(guild_id: Option<GuildId>, member_id: UserId, reason: String, by_id: UserId) -> Self::Data {
        MemberMute {
            reason,
            member: member_id,
            by: by_id,
            guild_id: guild_id,
            kind: String::from(Self::TYPE),
            time: Utc::now(),
            duration: None,
            unmute: None,
        }
    }
}

impl MemberMute {
    pub fn set_duration(&mut self, duration: DateTime<Utc>) {
        self.duration = Some(duration);
    }

    pub fn get_formated_duration(&self, offset: &FixedOffset) -> String {
        match self.duration {
            Some(time) => time.with_timezone::<FixedOffset>(offset).format_localized("%A, dia %d de %B de %Y, as %X (%Z)", Locale::pt_BR).to_string(),
            None => String::from("tempo indefinido")
        }
    }

    pub async fn unmute(&mut self, database: &Mongodb, by_id: UserId, reason: Option<String>) -> Result<(), Box<dyn Error>> {
        let unmute = Unmute {
            by: by_id,
            time: Utc::now(),
            reason
        };

        let query = doc! {
            "member": self.member.serialize(Serializer::new())?,
            "unmute": Bson::Null,
        };

        let update = doc! {
            "$set": {
                "unmute": unmute.serialize(Serializer::new())?
            }
        };

        database.update_one::<MemberMute>("Logger", "punishment", query, update).await?;

        Ok(())
    }
}