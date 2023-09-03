use std::collections::HashMap;
use bson::{doc, Document};

use axum::{
    http::StatusCode,
    extract::State,
    Json
};
use serde::Serialize;

use crate::{cache::ConfigManagerCache, api::router::RouterData};


#[derive(Serialize)]
pub struct ResBody {
    msg: String,
    data: Option<HashMap<String, Document>>
}

pub async fn get_configs(State(state): State<RouterData>) -> (StatusCode, Json<ResBody>) {
    let map = state.bot_data.read().await;
    let cfg_manager = match map.get::<ConfigManagerCache>() {
        Some(cfg_mngr) => cfg_mngr.read().await,
        None => {
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(ResBody { msg: "CONFIG_MANAGER_NOT_FOUND".to_string(), data: None }))
        }
    };

    let configs = cfg_manager.get_all_configs();

    (StatusCode::OK, Json(ResBody { msg: "ok".to_string(), data: Some(configs) }))
}