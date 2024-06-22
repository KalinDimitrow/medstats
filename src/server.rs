use axum::Router;
use tokio::signal;
use std::time::Duration;
use tower_http::timeout::TimeoutLayer;

use crate::endpoints::hellowold;
use crate::endpoints::test;

fn tracing_config() {
    tracing_subscriber::fmt::init();
}

fn create_app() -> Router {
    Router::new()
        .merge(hellowold::hello_router())
        .merge(test::test_router())
        .layer((
        // Graceful shutdown will wait for outstanding requests to complete. Add a timeout so
        // requests don't hang forever.
        TimeoutLayer::new(Duration::from_secs(10)),
    ))
}
pub async fn server() -> Result<(), Box<dyn std::error::Error>> {
    // initialize tracing
    tracing_config();

    // build our application with a route
    let app = create_app();
    
    // run our app with hyper
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).with_graceful_shutdown(shutdown_signal()).await?;

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
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
