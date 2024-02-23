use crate::{log_util::LoggableOutcome, web::{dto::{car_claims::CarClaims, Claim}, extractors::{token::Token, validate_body::ValidatedJson}, AppState}};
use axum::{extract::State, http::{HeaderMap, StatusCode}, Json};
use serde::Deserialize;
use serde_json::{json, Value};
use validator::Validate;
use crate::web::errors::HttpError;

#[derive(Deserialize, Validate)]
pub struct LowFuelRequest{
    value: u8
}

/*
    the bridge will call this route to trigger the notification 
    that will be sent to the user's phone
*/
pub async fn handler(
    State(s): State<AppState>,
    Token(token): Token<Claim<CarClaims>>,
    headers: HeaderMap,
    ValidatedJson(body): ValidatedJson<LowFuelRequest>,
) -> Result<Json<Value>, HttpError> {

    let mut conn = s.redis
        .get_async_connection()
        .await
        .log_err_to_error("couldn't get redis connection")?;

    if let Some(jwt_header) = headers.get("Authorization") {
        let jwt_split: Vec<&str> = jwt_header.to_str().unwrap().split("Bearer ").collect(); // this should be ok, since Token(token) passed
        
        let token_data = token.data();
        redis::cmd("XADD")
            .arg(s.stream_name)
            .arg("*")
            .arg("user_id")
            .arg(&token_data.owner)
            .arg("value")
            .arg(body.value)
            .arg("car_token")
            .arg(jwt_split[1])
            .query_async(&mut conn)
            .await?;

        Ok(Json(
            json!({ "success": true, "message": "topic_updated" }),
        ))
    }else{
        Err(HttpError::Simple(StatusCode::BAD_REQUEST, "huh?".to_string()))
    }

}
