mod errors;
pub mod extractors;
pub mod middlewares;
mod routes;

use std::{env, time::Duration};

use axum::{extract::FromRef, routing::post, Router};
use log::info;
use sqlx::postgres::{PgPoolOptions, Postgres};

#[derive(Clone, FromRef)]
pub struct AppState {
    pool: sqlx::Pool<Postgres>,
}
impl AppState {
    pub async fn new() -> Result<AppState, anyhow::Error> {
        info!("acquiring pool...");

        let pool = PgPoolOptions::new()
            .acquire_timeout(Duration::from_secs(3))
            .max_connections(1)
            .connect(&env::var("CONN_URI").unwrap())
            .await?;

        Ok(AppState { pool })
    }
}

pub async fn build_app() -> Router {
    let state = AppState::new().await.unwrap();
    info!("state ok");
    let app = Router::new()
        .route("/", post(routes::main::root::handler))
        .with_state(state);
    app
}
