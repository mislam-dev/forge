use axum::{extract::Request, http::HeaderValue, middleware::Next, response::Response};
use std::{process, time::Instant};
use tracing::{Instrument, error, info};
use tracing_appender::{non_blocking::WorkerGuard, rolling};
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};
use url::Url;
use uuid::Uuid;

pub fn init_tracing(rust_log: &str) -> WorkerGuard {
    let filter = EnvFilter::try_new(rust_log).unwrap_or_else(|_| EnvFilter::new("info"));

    let file_appender = rolling::daily("logs", "forge.log");
    let (non_blocking_writer, guard) = tracing_appender::non_blocking(file_appender);

    let loki = init_loki().ok();

    tracing_subscriber::registry()
        .with(filter)
        .with(fmt::layer().with_target(true).pretty())
        .with(
            fmt::layer()
                .json()
                .with_target(true)
                .with_current_span(true)
                .with_span_list(true)
                .with_writer(non_blocking_writer),
        )
        .with(loki)
        .init();

    guard
}

fn init_loki() -> Result<tracing_loki::Layer, tracing_loki::Error> {
    let (layer, task) = tracing_loki::builder()
        .label("app", "forge")?
        .label("env", "development")?
        .extra_field("pid", format!("{}", process::id()))?
        .build_url(Url::parse("http://127.0.0.1:3100").unwrap())?;

    tokio::spawn(task);

    Ok(layer)
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
