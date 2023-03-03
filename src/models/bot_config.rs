use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct BotConfig {
    id: String,
    name: String,
    data: String,
}