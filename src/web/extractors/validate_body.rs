use async_trait::async_trait;
use axum::{
    extract::{rejection::JsonRejection, FromRequest},
    http::{Request, StatusCode},
    Json,
};
use validator::Validate;

use crate::web::errors::HttpError;

pub struct ValidatedJson<T>(pub T);

#[async_trait]
impl<S, B, T> FromRequest<S, B> for ValidatedJson<T>
where
    axum::Json<T>: FromRequest<S, B, Rejection = JsonRejection>,
    S: Send + Sync,
    T: Validate,
    B: Send + 'static,
{
    type Rejection = HttpError;

    async fn from_request(
        req: Request<B>,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let (parts, body) = req.into_parts();
        let rebuilt_request = Request::from_parts(parts, body); // we need to rebuild the request since the Json extractor
                                                                // already consumed the body!
        let Json(j) = axum::Json::<T>::from_request(rebuilt_request, state)
            .await
            .map_err(|_| {
                HttpError::ParsingError(
                    "invalid_body".to_owned(),
                    StatusCode::BAD_REQUEST,
                )
            })?;

        j.validate()?;

        Ok(Self(j))
    }
}
