use crate::web::{extractors::validate_body::ValidatedJson, AppState};
use axum::{extract::State, Json};
use serde::Deserialize;
use serde_json::{json, Value};
use sqlx::Acquire;
use validator::Validate;

use crate::web::errors::HttpError;

#[derive(Deserialize, Validate)]
pub struct Test {
    #[validate(range(min = 1, max = 100))]
    t: i32,
}

pub async fn handler(
    State(s): State<AppState>, // debug handler
    ValidatedJson(body): ValidatedJson<Test>,
) -> Result<Json<Value>, HttpError> {
    let mut conn = s.pool.acquire().await?;
    let mut tx = conn.begin().await?;

    /* stuff */

    tx.commit().await?;

    Ok(Json(
        json!({ "success": true, "message": "welcome to dockerissimo-rust" }),
    ))
}
