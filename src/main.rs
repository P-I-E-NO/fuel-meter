mod log_util;
mod web;

use std::{net::SocketAddr, env::var};
use log::info;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let port = var("HTTP_PORT").unwrap().parse::<u16>().unwrap();
    let router = web::build_app().await;
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("{app} built and running on port {p}", app = var("APP_NAME").unwrap_or("fuel-meter-svc".to_string()), p = port);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, router).await.unwrap();
}

#[cfg(test)]
mod tests {
    use axum::{http::{Request, header::AUTHORIZATION, StatusCode}, body::Body};
    use serde_json::Value;
    use crate::web::build_test_router;

    use super::web::build_app;
    use tower::{Service, ServiceExt};

    #[tokio::test]
    async fn no_auth_header() {
        let app = build_test_router();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/test-auth-middleware")
                    .body(Body::empty()).unwrap()
                )
            .await
            .unwrap();
        
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        dbg!(response.body());

    }
}