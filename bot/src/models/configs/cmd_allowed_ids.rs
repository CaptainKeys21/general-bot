use serde::{Deserialize, Serialize};
use bson::doc;

use crate::models::traits::GeneralBotConfig;

#[derive(Deserialize, Serialize, Clone)]
pub struct CmdAllowedIds{
    config_type: String,
    name: String,
    group: String,
    allowed: Allowed,
}

#[derive(Deserialize, Serialize, Clone)]
struct Allowed {
    roles: Vec<u64>,
    users: Vec<u64>,
}

impl GeneralBotConfig for CmdAllowedIds{
    type Data = CmdAllowedIds;
    const TYPE: &'static str = "command_allowed_ids";
}

impl CmdAllowedIds{
    pub fn get_all_ids(&self) -> Vec<u64> {
        [&self.allowed.roles[..], &self.allowed.users[..]].concat()
    }
}