use std::sync::Arc;
use bson::doc;

use axum::{
    http::StatusCode,
    extract::State,
    Json
};
use serde::{Deserialize, Serialize};
use serenity::prelude::{RwLock, TypeMap};

use crate::{cache::{DatabaseCache, ConfigCache}, models::{general_config::GeneralConfig, traits::UpdateFromDataBase}};


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
    let database = map.get::<DatabaseCache>().unwrap().read().await;

    if let Err(e) = GeneralConfig::edit_one(&database, payload.prefix.clone(), doc! {"name": "prefix"}).await {
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(ResBody { msg: e.to_string() }))
    };

    {
        let mut map_config = map.get::<ConfigCache>().unwrap().write().await;
        map_config.insert("BOT_PREFIX", payload.prefix);
    }

    (StatusCode::OK, Json(ResBody { msg: String::from("Prefix changed") }))
}