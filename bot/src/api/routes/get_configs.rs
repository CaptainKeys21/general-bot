use std::{sync::Arc, collections::HashMap};
use bson::{doc, Document};

use axum::{
    http::StatusCode,
    extract::State,
    Json
};
use serde::Serialize;
use serenity::prelude::{RwLock, TypeMap};

use crate::cache::ConfigManagerCache;


#[derive(Serialize)]
pub struct ResBody {
    msg: String,
    data: Option<HashMap<String, Document>>
}

pub async fn get_configs(State(data): State<Arc<RwLock<TypeMap>>>) -> (StatusCode, Json<ResBody>) {
    let map = data.read().await;
    let cfg_manager = match map.get::<ConfigManagerCache>() {
        Some(cfg_mngr) => cfg_mngr.read().await,
        None => {
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(ResBody { msg: "CONFIG_MANAGER_NOT_FOUND".to_string(), data: None }))
        }
    };

    let configs = cfg_manager.get_all_configs();

    (StatusCode::OK, Json(ResBody { msg: "ok".to_string(), data: Some(configs) }))
}