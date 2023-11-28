use crate::web::{extractors::validate_body::ValidatedJson, AppState, middlewares::auth::Claims};
use axum::{extract::State, Json, Extension};
use jsonwebtoken::TokenData;
use serde::Deserialize;
use serde_json::{json, Value};
use validator::Validate;
use redis::AsyncCommands;

use crate::web::errors::HttpError;

#[derive(Deserialize, Validate)]
pub struct Test {
    value: u8,
}

pub async fn handler(
    State(s): State<AppState>, // debug handler
    Extension(jwt): Extension<TokenData<Claims>>,
    ValidatedJson(body): ValidatedJson<Test>,
) -> Result<Json<Value>, HttpError> {
    let mut conn = s.redis.get_async_connection().await?;
    
    redis::cmd("XADD")
        .arg("streams:notifications")
        .arg("*")
        .arg("user_id")
        .arg(jwt.claims.user_id)
        .arg("value")
        .arg(body.value)
        .query_async(&mut conn)
        .await?;

    Ok(Json(
        json!({ "success": true, "message": "topic updated" }),
    ))
}
