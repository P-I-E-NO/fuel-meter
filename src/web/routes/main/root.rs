use std::env::var;

use crate::{web::{extractors::validate_body::ValidatedJson, AppState, middlewares::auth::Claims}, log_util::LoggableOutcome};
use axum::{extract::State, Json, Extension};
use jsonwebtoken::TokenData;
use serde::Deserialize;
use serde_json::{json, Value};
use validator::Validate;
use crate::web::errors::HttpError;

#[derive(Deserialize, Validate)]
pub struct Test {
    value: u8,
    lat: String,
    lon: String
}

/*
    the bridge will call this route to trigger the notification 
    that will be sent to the user's phone
*/
pub async fn handler(
    State(s): State<AppState>,
    Extension(jwt): Extension<TokenData<Claims>>,
    ValidatedJson(body): ValidatedJson<Test>,
) -> Result<Json<Value>, HttpError> {

    let mut conn = s.redis
        .get_async_connection()
        .await
        .log_err_to_error("couldn't get redis connection")?;
    
    redis::cmd("XADD")
        .arg(s.stream_name)
        .arg("*")
        .arg("user_id")
        .arg(jwt.claims.user_id)
        .arg("value")
        .arg(body.value)
        .arg("lat")
        .arg(body.lat)
        .arg("lon")
        .arg(body.lon)
        .query_async(&mut conn)
        .await?;

    Ok(Json(
        json!({ "success": true, "message": "topic_updated" }),
    ))

}
