mod errors;
pub mod extractors;
pub mod middlewares;
mod routes;

use std::env;
use axum::{routing::post, Router, middleware};
use log::info;
use env::var;

use crate::web::middlewares::auth::auth;
#[derive(Clone)]
pub struct AppState {
    redis: redis::Client,
    stream_name: String,
}
impl AppState {
    pub async fn new() -> Result<AppState, anyhow::Error> {
        info!("connecting to redis");
        let redis = redis::Client::open(var("REDIS_URL").unwrap()).unwrap();
        Ok(AppState { redis, stream_name: var("REDIS_STREAM").unwrap() })
    }
}

pub async fn build_app() -> Router {
    let state = AppState::new().await.unwrap();
    build_router(state)
}

pub fn build_router(state: AppState) -> Router {
    let app = Router::new()
        .route("/", post(routes::main::root::handler))
        .layer(middleware::from_fn_with_state(state.clone(), auth))
        .with_state(state);
    app
}