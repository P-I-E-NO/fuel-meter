use std::env;

use async_trait::async_trait;
use axum::{extract::FromRequestParts, http::{header::AUTHORIZATION, request::Parts, StatusCode}};
use jsonwebtoken::{encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{de::DeserializeOwned, Serialize};

use crate::web::{dto::{car_claims::CarClaims, Claim}, errors::HttpError};

pub struct Token<T: Send + Serialize + 'static>(pub T);

impl<T> Token<T> 
    where T: DeserializeOwned + Send + Serialize + 'static // essentially, T must not have lifetimes inside, because it needs
                                                           // to be moved!
{
    pub async fn generate(item: T) -> Result<String, HttpError> {
        let key = env::var("JWT_SECRET").unwrap();  
        
        tokio::task::spawn_blocking(move || {
            Ok(
                encode(
                    &Header::default(),
                    &item, 
                    &EncodingKey::from_secret(&key.as_bytes())
                )?
            )
        }).await?

    }
    pub async fn from(token: &str) -> Result<Token<T>, HttpError> {
        let key = env::var("JWT_SECRET").unwrap();
        let validation = Validation::new(Algorithm::HS256);
        let token = token.to_string(); // we need the owned version because spawn_blocking moves 

        tokio::task::spawn_blocking(move || {
            Ok(
                Token(
                    jsonwebtoken::decode::<T>(
                            &token, 
                            &DecodingKey::from_secret(&key.as_bytes()),
                            &validation
                    )?.claims
                )
            )
        }).await?
        
    }
}

// we need to implement FromRequestParts for every Token (Claim<UserClaims>, ...) we can "see" in the request headers
#[async_trait]
impl<S> FromRequestParts<S> for Token<Claim<CarClaims>>
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
                    let str_token = pieces[1];
                    Ok(
                        Token::from(str_token).await?
                    )
                } else {
                    Err(HttpError::Simple(StatusCode::BAD_REQUEST, "invalid_auth_header".to_string()))
                }
        } else {
            Err(HttpError::Simple(StatusCode::BAD_REQUEST, "no_auth_header".to_string()))
        }
    }
}