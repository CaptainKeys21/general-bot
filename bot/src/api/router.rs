use std::sync::Arc;

use axum::{
    routing::{
        get,
        put,
    },
    Router
};
use serenity::prelude::{RwLock, TypeMap};

use crate::api::routes::{
    root::root,
    prefix::update_prefix,
    logger_blocklist::set_blocklist,
    get_configs::get_configs,
    timezone::set_timezone
};

#[derive(Clone)]
pub struct RouterData {
    pub bot_data: Arc<RwLock<TypeMap>>
}

pub fn build_router(data: RouterData) -> Router {
    let router = Router::new()
        .route("/api/", get(root))
        .route("/api/configs", get(get_configs))
        .route("/api/prefix", put(update_prefix))
        .route("/api/logger_blocklist", put(set_blocklist))
        .route("/api/timezone", put(set_timezone))
        .with_state(data);

    router
}