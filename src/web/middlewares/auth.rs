use axum::{response::Response, http::{Request, StatusCode}, middleware::Next, body::Body};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Serialize, Deserialize};

use crate::web::{errors::HttpError, extractors::token::Token};
use std::env::var;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub user_id: String,
    pub exp: usize
}

pub async fn auth(
    Token(auth): Token,
    mut request: Request<Body>,
    next: Next,
) -> Result<Response, HttpError> {

    match decode::<Claims>(
        &auth,
        &DecodingKey::from_secret(var("JWT_KEY").unwrap().as_ref()), 
        &Validation::default()
    ) {
        Ok(token) => {
            request.extensions_mut().insert(token);
            let response = next.run(request).await;
            Ok(response)
        }
        Err(e) => {
            match  *e.kind() {
                jsonwebtoken::errors::ErrorKind::InvalidToken => Err(HttpError::ParsingError("invalid_jwt_token".to_string(), StatusCode::BAD_REQUEST)),
                jsonwebtoken::errors::ErrorKind::InvalidSignature => Err(HttpError::ParsingError("invalid_jwt_signature".to_string(), StatusCode::BAD_REQUEST)),
                jsonwebtoken::errors::ErrorKind::InvalidEcdsaKey => Err(HttpError::ParsingError("invalid_jwt_ecdsa_key".to_string(), StatusCode::BAD_REQUEST)),
                jsonwebtoken::errors::ErrorKind::MissingRequiredClaim(_) => Err(HttpError::ParsingError("missing_jwt_claim".to_string(), StatusCode::BAD_REQUEST)),
                jsonwebtoken::errors::ErrorKind::ExpiredSignature => Err(HttpError::ParsingError("expired_jwt".to_string(), StatusCode::BAD_REQUEST)),
                _ => Err(HttpError::ParsingError("bad_jwt".to_string(), StatusCode::BAD_REQUEST)),
            }
        }
        
    }

}