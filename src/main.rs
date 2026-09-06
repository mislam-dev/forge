use std::net::SocketAddr;

use tokio::signal;
use tower_http::trace::TraceLayer;

use forge::{
    app::{app::create_app, state::AppState},
    config::AppConfig,
    infrastructure::queue::{RabbitMq, RabbitMqConfig, RabbitMqTopology},
    shared::logger,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app_config = AppConfig::load()?;

    let log_filter = if app_config.server_config.rust_log {
        "debug"
    } else {
        "info"
    };

    let _guard = logger::init_tracing(log_filter);

    tracing::info!("Starting appplication.....");
    let app_state = AppState::new().await?;

    let rmq_config = RabbitMqConfig::from_env();

    match RabbitMq::connect(&rmq_config).await {
        Ok(rmq) => {
            if let Err(err) = RabbitMqTopology::setup(&rmq).await {
                tracing::error!(error = %err, "Failed to declare RabbitMQ topology");
            } else {
                tracing::info!("RabbitMQ topology verified successfully");
            }
        }
        Err(e) => {
            tracing::warn!(error = %e, "Could not connect to RabbitMQ broker on startup");
        }
    }

    let app = create_app(app_state).await?;

    let host = app_config.server_config.server_host;
    let port = app_config.server_config.server_port;

    let addr = SocketAddr::new(host, port);

    let listener = tokio::net::TcpListener::bind(addr).await?;

    let local_addr = listener.local_addr().unwrap();

    tracing::info!("Server listening on {}", local_addr);

    let _ = axum::serve(listener, app.layer(TraceLayer::new_for_http()))
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            tracing::info!("Ctrl+C received! Shutting down gracefully...");
        },
        _ = terminate => {
            tracing::info!("Terminate signal received! Shutting down gracefully...");
        },
    }
}
