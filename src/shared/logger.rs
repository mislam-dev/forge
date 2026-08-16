use axum::{extract::Request, http::HeaderValue, middleware::Next, response::Response};
use std::time::Instant;
use tracing::{Instrument, error, info};
use tracing_subscriber::{EnvFilter, fmt};
use uuid::Uuid;

pub fn init_tracing(rust_log: &str) {
    let filter = EnvFilter::try_new(rust_log).unwrap_or_else(|_| EnvFilter::new("info"));

    fmt()
        .json()
        .with_env_filter(filter)
        .with_target(true)
        .with_current_span(true)
        .with_span_list(true)
        .init();
}

const REQUEST_ID_HEADER: &str = "x-request-id";

pub async fn logging_middleware(mut req: Request, next: Next) -> Response {
    let request_id = req
        .headers()
        .get(REQUEST_ID_HEADER)
        .and_then(|v| v.to_str().ok())
        .map(String::from)
        .unwrap_or_else(|| Uuid::new_v4().to_string());

    req.headers_mut().insert(
        REQUEST_ID_HEADER,
        HeaderValue::from_str(&request_id).unwrap(),
    );

    let method = req.method().clone();
    let path = req.uri().path().to_string();

    let span = tracing::info_span!(
        "http_request",
        request_id = %request_id,
        method = %method,
        path = %path,
    );

    let start = Instant::now();

    async move {
        info!("request.started");

        let mut response = next.run(req).await;

        let latency_ms = start.elapsed().as_millis();
        let status = response.status();

        response.headers_mut().insert(
            REQUEST_ID_HEADER,
            HeaderValue::from_str(&request_id).unwrap(),
        );

        if status.is_server_error() {
            error!(status = %status, latency_ms, "request.completed");
        } else {
            info!(status = %status, latency_ms, "request.completed");
        }

        response
    }
    .instrument(span)
    .await
}
