use std::collections::HashMap;

use axum::{
    extract::rejection::JsonRejection,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use redis::RedisError;
use serde_json::json;
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
    fn from(err: JsonRejection) -> Self {
        HttpError::ParsingError("invalid_body".to_owned(), err.status())
    }
}

impl From<ValidationErrors> for HttpError {
    fn from(err: ValidationErrors) -> Self {
        Self::InvalidFieldsError(err.into_errors())
    }
}

impl From<RedisError> for HttpError {
    fn from(_: RedisError) -> Self {
        Self::Simple(StatusCode::INTERNAL_SERVER_ERROR, "fatal_error".to_string())
    }
}