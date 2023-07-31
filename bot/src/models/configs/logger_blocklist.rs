use serde::{Deserialize, Serialize};
use bson::doc;

use crate::models::traits::GeneralBotConfig;

#[derive(Deserialize, Serialize, Clone)]
pub struct LoggerBlocklist{
    pub config_type: String,
    pub name: String,
    pub blocked: Blocked,
}

#[derive(Deserialize, Serialize, Clone, Default)]
pub struct Blocked {
    roles: Vec<String>,
    users: Vec<String>,
}

impl GeneralBotConfig for LoggerBlocklist{
    type Data = LoggerBlocklist;
    const TYPE: &'static str = "logger";
}

impl Default for LoggerBlocklist {
    fn default() -> Self {
        LoggerBlocklist { 
            config_type: String::from(LoggerBlocklist::TYPE), 
            name: String::from(LoggerBlocklist::NAME), 
            blocked: Blocked::default(),
        }
    }
}

impl LoggerBlocklist{
    pub const NAME: &'static str = "blocklist";
    pub fn get_all_ids(&self) -> Vec<String> {
        [&self.blocked.roles[..], &self.blocked.users[..]].concat()
    }
}