mod errors;
pub mod extractors;
pub mod middlewares;
mod routes;

use std::env;
use axum::{extract::FromRef, routing::post, Router, middleware};
use log::info;
use env::var;

use crate::web::middlewares::auth::auth;
#[derive(Clone, FromRef)]
pub struct AppState {
    redis: redis::Client
}
impl AppState {
    pub async fn new() -> Result<AppState, anyhow::Error> {
        info!("connecting to redis");
        let redis = redis::Client::open(var("REDIS_URL").unwrap()).unwrap();
        Ok(AppState { redis })
    }
}

pub async fn build_app() -> Router {
    let state = AppState::new().await.unwrap();
    let app = Router::new()
        .route("/", post(routes::main::root::handler))
        .layer(middleware::from_fn_with_state(state.clone(), auth))
        .with_state(state);
    app
}
