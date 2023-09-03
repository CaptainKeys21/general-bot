use bson::doc;

use axum::{
    http::StatusCode,
    extract::State,
    Json
};
use serde::{Deserialize, Serialize};

use crate::{cache::ConfigManagerCache, models::{configs::logger_blocklist::{LoggerBlocklist, Blocked}, traits::GeneralBotConfig}, api::router::RouterData};


#[derive(Deserialize)]
pub struct ReqBlocklist {
    pub ids: Blocked,
}

#[derive(Serialize)]
pub struct ResBody {
    msg: String,
}

pub async fn set_blocklist(State(state): State<RouterData>, Json(payload): Json<ReqBlocklist>) -> (StatusCode, Json<ResBody>) {
    let map = state.bot_data.read().await;
    let mut cfg_manager = match map.get::<ConfigManagerCache>() {
        Some(cfg_mngr) => cfg_mngr.write().await,
        None => {
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(ResBody { msg: "CONFIG_MANAGER_NOT_FOUND".to_string() }))
        }
    };

    let blocklist_data = LoggerBlocklist {
        name: LoggerBlocklist::NAME.to_string(),
        config_type: LoggerBlocklist::TYPE.to_string(),
        blocked: payload.ids,
    };

    if let Err(e) = cfg_manager.update_one::<LoggerBlocklist>(LoggerBlocklist::NAME, blocklist_data).await {
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(ResBody { msg: e.to_string() }))
    };

    (StatusCode::OK, Json(ResBody { msg: String::from("Blocklist changed") }))
}