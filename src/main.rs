use std::{error::Error, net::SocketAddr};

use axum::Router;
use canvas_rs::config;
use clap::Parser;
use sqlx::PgPool;
use tokio::signal;
use tower_http::services::ServeDir;
mod handlers;
mod models;
mod templates;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // load environment variable from .env file
    dotenv::dotenv().ok();

    // load the env variable into config struct
    let config = config::Config::parse();

    // initialize the database pool
    let pool = PgPool::connect(&config.database_url).await?;

    // build our application with a route
    let app = Router::new()
        .nest_service("/", handlers::router(pool))
        .nest_service("/public", ServeDir::new("public"));

    // bind an address from the env port
    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));

    // run it
    let listener = tokio::net::TcpListener::bind(addr).await?;
    println!("listening on {}", listener.local_addr()?);
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

// graceful shutdown: this code is taken from
// https://github.com/tokio-rs/axum/blob/main/examples/graceful-shutdown/src/main.rs
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
