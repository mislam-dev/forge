use axum::http::Method;
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
        .allow_origin(Any);
    cors
}
