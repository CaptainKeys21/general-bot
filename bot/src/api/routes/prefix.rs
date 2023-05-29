use std::sync::Arc;
use bson::doc;

use axum::{
    http::StatusCode,
    extract::State,
    Json
};
use serde::{Deserialize, Serialize};
use serenity::prelude::{RwLock, TypeMap};

use crate::{cache::ConfigManagerCache, models::configs::general::GeneralConfig};


#[derive(Deserialize)]
pub struct ReqPrefix {
    pub prefix: String,
}

#[derive(Serialize)]
pub struct ResBody {
    msg: String,
}

pub async fn update_prefix(State(data): State<Arc<RwLock<TypeMap>>>, Json(payload): Json<ReqPrefix>) -> (StatusCode, Json<ResBody>) {
    let map = data.read().await;
    let mut cfg_manager = match map.get::<ConfigManagerCache>() {
        Some(cfg_mngr) => cfg_mngr.write().await,
        None => {
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(ResBody { msg: "CONFIG_MANAGER_NOT_FOUND".to_string() }))
        }
    };

    let prefix_data = GeneralConfig {
        name: String::from("prefix"),
        data: payload.prefix,
        config_type: String::from("general"),
    };

    if let Err(e) = cfg_manager.update_one::<GeneralConfig>("prefix", prefix_data).await {
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(ResBody { msg: e.to_string() }))
    };

    (StatusCode::OK, Json(ResBody { msg: String::from("Prefix changed") }))
}