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
    update_prefix::update_prefix,
};

pub fn build_router(data: Arc<RwLock<TypeMap>>) -> Router {
    let router = Router::new()
        .route("/bot_api/", get(root))
        .route("/bot_api/prefix", put(update_prefix))
        .with_state(data);

    router
}