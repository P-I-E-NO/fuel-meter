use axum::Extension;
use jsonwebtoken::TokenData;

use crate::web::middlewares::auth::Claims;

pub async fn auth_test(
    Extension(_): Extension<TokenData<Claims>>,
) -> &'static str {
    "ok"
}