use bson::doc;

use axum::{
    http::StatusCode,
    extract::State,
    Json
};
use serde::{Deserialize, Serialize};

use crate::{cache::ConfigManagerCache, models::{configs::general::GeneralConfig, traits::GeneralBotConfig}, api::router::RouterData};


#[derive(Deserialize)]
pub struct ReqBody {
    pub seconds: i32,
    pub hemisphere: String,
}

#[derive(Serialize)]
pub struct ResBody {
    msg: String,
}

pub async fn set_timezone(State(state): State<RouterData>, Json(payload): Json<ReqBody>) -> (StatusCode, Json<ResBody>) {
    let map = state.bot_data.read().await;
    let mut cfg_manager = match map.get::<ConfigManagerCache>() {
        Some(cfg_mngr) => cfg_mngr.write().await,
        None => {
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(ResBody { msg: "CONFIG_MANAGER_NOT_FOUND".to_string() }))
        }
    };

    if &payload.hemisphere != "W" && &payload.hemisphere !="E" {
        return (StatusCode::BAD_REQUEST, Json(ResBody { msg: "HEMISPHERE_NOT_VALID".to_string() }));
    }

    if payload.seconds < 0 || payload.seconds > 43200 {
        return (StatusCode::BAD_REQUEST, Json(ResBody { msg: "SECONDS_FIELD_IS_OUT_OF_RANGE".to_string() }));
    }

    let data = GeneralConfig {
        name: String::from("timezone"),
        data: payload.hemisphere + ":" + &payload.seconds.to_string(),
        config_type: GeneralConfig::TYPE.to_string(),
    };

    if let Err(e) = cfg_manager.update_one::<GeneralConfig>("timezone", data).await {
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(ResBody { msg: e.to_string() }))
    };

    (StatusCode::OK, Json(ResBody { msg: String::from("Timezone changed") }))
}