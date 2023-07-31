use serde::{Deserialize, Serialize};
use bson::doc;

use crate::models::traits::GeneralBotConfig;

#[derive(Deserialize, Serialize, Clone)]
pub struct CmdAllowedIds {
    config_type: String,
    name: String,
    // group: String,
    allowed: Allowed,
}

#[derive(Deserialize, Serialize, Clone, Default)]
struct Allowed {
    roles: Vec<String>,
    users: Vec<String>,
}

impl GeneralBotConfig for CmdAllowedIds {
    type Data = CmdAllowedIds;
    const TYPE: &'static str = "command_allowed_ids";
}

impl Default for CmdAllowedIds {
    fn default() -> Self {
        CmdAllowedIds { 
            config_type: String::from(CmdAllowedIds::TYPE), 
            name: String::default(), 
            allowed: Allowed::default()
        }
    }
}

impl CmdAllowedIds{
    pub fn get_all_ids(&self) -> Vec<String> {
        [&self.allowed.roles[..], &self.allowed.users[..]].concat()
    }
}