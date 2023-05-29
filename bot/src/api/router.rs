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
    get_configs::get_configs
};

pub fn build_router(data: Arc<RwLock<TypeMap>>) -> Router {
    let router = Router::new()
        .route("/api/", get(root))
        .route("/api/configs", get(get_configs))
        .route("/api/prefix", put(update_prefix))
        .route("/api/logger_blocklist", put(set_blocklist))
        .with_state(data);

    router
}