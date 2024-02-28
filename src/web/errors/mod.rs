use std::collections::HashMap;

use axum::{
    extract::rejection::JsonRejection,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use log::info;
use redis::RedisError;
use serde_json::json;
use tokio::task::JoinError;
use validator::{ValidationErrors, ValidationErrorsKind};

pub enum HttpError {
    ParsingError(String, StatusCode),
    InvalidFieldsError(HashMap<&'static str, ValidationErrorsKind>),
    Simple(StatusCode, String),
}

impl IntoResponse for HttpError {
    fn into_response(self) -> Response {
        let tuple_response = match self {
            HttpError::ParsingError(text, _) => (
                StatusCode::BAD_REQUEST,
                Json(json!({"success": false, "error": text})),
            ),
            HttpError::InvalidFieldsError(err) => {
                let invalid_fields: Vec<&str> =
                    err.into_keys().map(|i| i).collect();
                (
                    StatusCode::BAD_REQUEST,
                    Json(
                        json!({"success": false, "error": "invalid_fields", "fields": invalid_fields}),
                    ),
                )
            }
            HttpError::Simple(code, msg) => {
                (code, Json(json!({ "success": false, "error": msg })))
            }
        };

        tuple_response.into_response()
    }
}

impl From<JsonRejection> for HttpError {
    // error while parsing invalid json
    fn from(err: JsonRejection) -> Self {
        Self::ParsingError("invalid_body".to_owned(), err.status())
    }
}

impl From<ValidationErrors> for HttpError {
    // error when validating structs
    fn from(err: ValidationErrors) -> Self {
        Self::InvalidFieldsError(err.into_errors())
    }
}
impl From<JoinError> for HttpError {
    // this is tokio's blocking task error
    fn from(_: JoinError) -> Self {
        Self::Simple(StatusCode::INTERNAL_SERVER_ERROR, "async_error".to_string())
    }
}
impl From<jsonwebtoken::errors::Error> for HttpError {
    fn from(e: jsonwebtoken::errors::Error) -> Self {
        info!("{}", &e.to_string());
        Self::Simple(StatusCode::INTERNAL_SERVER_ERROR, "bad_jwt".to_string()) // ErrorKind implements Display!
    }
}
impl From<RedisError> for HttpError {
    fn from(_: RedisError) -> Self {
        Self::Simple(StatusCode::INTERNAL_SERVER_ERROR, "fatal_error".to_string())
    }
}