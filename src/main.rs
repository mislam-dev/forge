use std::net::SocketAddr;

use tokio::signal;
use tower_http::trace::TraceLayer;

use forge::{
    app::{app::create_app, state::AppState},
    config::AppConfig,
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
            println!("Ctrl+C received! Shutting down gracefully...");
        },
        _ = terminate => {
            println!("Terminate signal received! Shutting down gracefully...");
        },
    }
}
