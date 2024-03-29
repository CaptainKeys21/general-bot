use serde::{Deserialize, Serialize};

use crate::models::traits::GeneralBotConfig;

#[derive(Deserialize, Serialize)]
pub struct GeneralConfig {
    pub name: String,
    pub data: String,
    pub config_type: String,
}

impl GeneralBotConfig for GeneralConfig {
    type Data = GeneralConfig;
    const TYPE: &'static str = "general";
}

impl Default for GeneralConfig {
    fn default() -> Self {
        GeneralConfig { 
            name: String::default(), 
            data: String::default(), 
            config_type: String::from(GeneralConfig::TYPE) 
        }
    }
}