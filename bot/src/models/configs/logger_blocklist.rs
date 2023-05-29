use serde::{Deserialize, Serialize};
use bson::doc;

use crate::models::traits::GeneralBotConfig;

#[derive(Deserialize, Serialize, Clone)]
pub struct LoggerBlocklist{
    pub config_type: String,
    pub name: String,
    pub blocked: Blocked,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Blocked {
    roles: Vec<u64>,
    users: Vec<u64>,
}

impl GeneralBotConfig for LoggerBlocklist{
    type Data = LoggerBlocklist;
    const TYPE: &'static str = "logger";
}

impl LoggerBlocklist{
    pub const NAME: &'static str = "blocklist";
    pub fn get_all_ids(&self) -> Vec<u64> {
        [&self.blocked.roles[..], &self.blocked.users[..]].concat()
    }
}