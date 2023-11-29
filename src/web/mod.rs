mod errors;
pub mod extractors;
pub mod middlewares;
mod routes;

use std::env;
use axum::{extract::{FromRef, State}, routing::post, Router, middleware, http::Request};
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
    build_router(state)
}

pub fn build_router(state: AppState) -> Router {
    let app = Router::new()
        .route("/", post(routes::main::root::handler))
        .layer(middleware::from_fn_with_state(state.clone(), auth))
        .with_state(state);
    app
}

pub fn build_test_router() -> Router {
    let app = Router::new()
        .route(
            "/test-auth-middleware", 
            post(routes::test::auth_test).layer(middleware::from_fn(auth))
        );

    app
}