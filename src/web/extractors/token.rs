use async_trait::async_trait;
use axum::{extract::FromRequestParts, http::{request::Parts, header::AUTHORIZATION, StatusCode}};

use crate::web::errors::HttpError;

pub struct Token(pub String);

#[async_trait]
impl<S> FromRequestParts<S> for Token
where
    S: Send + Sync,
{
    type Rejection = HttpError;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        if let Some(auth_header) = parts.headers.get(AUTHORIZATION) {
            if let Ok(str_header) = auth_header
                .to_str() {
                    let pieces: Vec<&str> = str_header.split("Bearer ").collect();
                    if pieces.len() < 2 {
                        return Err(HttpError::Simple(StatusCode::BAD_REQUEST, "no_bearer_specified".to_string()))
                    }
                    Ok(Token(pieces[1].to_string()))
                } else {
                    Err(HttpError::Simple(StatusCode::BAD_REQUEST, "invalid_auth_header".to_string()))
                }
        } else {
            Err(HttpError::Simple(StatusCode::BAD_REQUEST, "no_auth_header".to_string()))
        }
    }
}
