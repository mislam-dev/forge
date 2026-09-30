use axum::http::HeaderName;
use axum::http::Method;
use axum::http::header;
use tower_http::cors::{Any, CorsLayer};

pub fn cors_middleware() -> CorsLayer {
    let cors = CorsLayer::new()
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::OPTIONS,
            Method::DELETE,
            Method::PATCH,
        ])
        .allow_origin(Any)
        .allow_headers([
            header::CONTENT_TYPE,
            header::AUTHORIZATION,
            header::ACCEPT,
            HeaderName::from_static("x-request-id"),
            HeaderName::from_static("organization-id"),
        ])
        .expose_headers([HeaderName::from_static("x-request-id")]);
    cors
}
